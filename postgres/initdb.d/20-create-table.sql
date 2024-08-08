-- Copyright © 2024 Akira Miyakoda
--
-- This software is released under the MIT License.
-- https://opensource.org/licenses/MIT

--------------------------------------------------------------------------------
-- Users
--------------------------------------------------------------------------------

CREATE TABLE users (
    user_id     UUID NOT NULL DEFAULT UUID_GENERATE_V7(),
    auth_id     TEXT NOT NULL UNIQUE,
    auth_hash   TEXT NOT NULL,
    social_id   TEXT NULL,
    appendix    JSONB NOT NULL DEFAULT '{}'::JSONB,
    created_at  TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at  TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),

    PRIMARY KEY (user_id)
);

--------------------------------------------------------------------------------
-- Sensors
--------------------------------------------------------------------------------

CREATE TABLE sensors (
    sensor_id   UUID NOT NULL DEFAULT UUID_GENERATE_V7(),
    user_id     UUID NULL,
    auth_hash   TEXT NOT NULL,
    appendix    JSONB NOT NULL DEFAULT '{}'::JSONB,
    created_at  TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at  TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),

    PRIMARY KEY (sensor_id),
    FOREIGN KEY (user_id) REFERENCES users (user_id)
);

CREATE INDEX ON sensors (user_id);

--------------------------------------------------------------------------------
-- Readings
--------------------------------------------------------------------------------

CREATE TABLE readings (
    sensor_id   UUID NOT NULL,
    read_at     BIGINT NOT NULL DEFAULT (EXTRACT(EPOCH FROM CURRENT_TIMESTAMP(3)) * 1000)::BIGINT,
    temp_in     DOUBLE PRECISION NOT NULL,
    temp_out    DOUBLE PRECISION NOT NULL,
    tds         DOUBLE PRECISION NOT NULL,

    PRIMARY KEY (sensor_id, read_at),
    FOREIGN KEY (sensor_id) REFERENCES sensors (sensor_id)
);

--------------------------------------------------------------------------------
-- Socket.IO sessions
--------------------------------------------------------------------------------

CREATE TABLE socketio_sessions (
    token       UUID NOT NULL DEFAULT GEN_RANDOM_UUID(),
    user_id     UUID NOT NULL,
    expires_at  TIMESTAMPTZ(6) NOT NULL,
    created_at  TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at  TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),

    PRIMARY KEY (token)
);

CREATE INDEX ON socketio_sessions (expires_at);
