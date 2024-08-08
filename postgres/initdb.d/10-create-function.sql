-- Copyright © 2024 Akira Miyakoda
--
-- This software is released under the MIT License.
-- https://opensource.org/licenses/MIT

--------------------------------------------------------------------------------
-- UUID_GENERATE_V7
--------------------------------------------------------------------------------

-- Based on RFC9562, https://datatracker.ietf.org/doc/html/rfc9562

--  0                   1                   2                   3
--  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
-- +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
-- |                           unix_ts_ms                          |
-- +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
-- |          unix_ts_ms           |  ver  |       rand_a          |
-- +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
-- |var|                        rand_b                             |
-- +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
-- |                            rand_b                             |
-- +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

CREATE FUNCTION uuid_generate_v7()
RETURNS UUID
AS $$
DECLARE
    unix_ts_ms BYTEA;
    uuid_bytes BYTEA;
BEGIN
    unix_ts_ms = SUBSTRING(INT8SEND(FLOOR(EXTRACT(EPOCH FROM CLOCK_TIMESTAMP()) * 1000)::BIGINT) FROM 3);
    uuid_bytes = UUID_SEND(GEN_RANDOM_UUID());
    uuid_bytes = OVERLAY(uuid_bytes PLACING unix_ts_ms FROM 1 FOR 6);
    uuid_bytes = SET_BYTE(uuid_bytes, 6, (B'0111' || GET_BYTE(uuid_bytes, 6)::BIT(4))::BIT(8)::INT);

    RETURN ENCODE(uuid_bytes, 'HEX')::UUID;
END
$$ LANGUAGE plpgsql
VOLATILE;

--------------------------------------------------------------------------------
-- TRIMMEAN
--------------------------------------------------------------------------------

CREATE TYPE trimmean_stype AS (
    bucket     FLOAT [],
    proportion FLOAT
);

CREATE FUNCTION trimmean_sfunc(trimmean_stype, float, float)
RETURNS trimmean_stype AS $$
    SELECT ARRAY_APPEND($1.bucket, $2), $3;
$$ LANGUAGE SQL STRICT
IMMUTABLE;

CREATE FUNCTION trimmean_finalfunc(trimmean_stype)
RETURNS float AS $$
DECLARE
    trim_count INT;
    data_count INT;
    result     FLOAT;
BEGIN
    IF $1.proportion < 0 OR 1.0 <= $1.proportion THEN
        return NULL;
    END IF;

    data_count := ARRAY_LENGTH($1.bucket, 1);
    trim_count := FLOOR(data_count * $1.proportion) / 2;
    IF trim_count > 0 THEN
        data_count := data_count - trim_count * 2;
        SELECT AVG(v) INTO result FROM (SELECT v FROM UNNEST($1.bucket) AS v ORDER BY v OFFSET trim_count LIMIT data_count) AS x;
    ELSE
        SELECT AVG(v) INTO result FROM UNNEST($1.bucket) AS v;
    END IF;

    RETURN result;
END;
$$ LANGUAGE plpgsql STRICT
IMMUTABLE;

CREATE AGGREGATE trimmean(float, float)
(
    SFUNC = trimmean_sfunc,
    STYPE = trimmean_stype,
    FINALFUNC = trimmean_finalfunc,
    INITCOND = '({}, 0)'
);
