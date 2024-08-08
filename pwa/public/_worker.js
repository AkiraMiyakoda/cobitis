// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// Worker for Cloudflare pages

export default {
    async fetch(request, env) {
        // Redirect API requests to api subdomain
        const url = new URL(request.url);
        if (url.pathname.startsWith("/api/")) {
            url.host = `api.${url.host}`;
            url.pathname = url.pathname.substring(4);
            return fetch(
                new Request(url.href, {
                    method: request.method,
                    headers: request.headers,
                    credentials: request.credentials,
                    body: request.body,
                })
            );
        }

        return env.ASSETS.fetch(request);
    },
};
