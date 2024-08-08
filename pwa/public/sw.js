// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// Built at: __BUILD_DATE__

const CACHE_VERSION = __CACHE_VERSION__;
const CACHE_FILES = [__CACHE_FILES__];

self.addEventListener("install", (event) => {
    event.waitUntil(
        caches.open(CACHE_VERSION).then((cache) => {
            cache.addAll(CACHE_FILES);
        })
    );
});

self.addEventListener("activate", function (event) {
    event.waitUntil(
        caches.keys().then((keys) => {
            return Promise.all(keys.filter((key) => key != CACHE_VERSION).map((key) => caches.delete(key)));
        })
    );
});

self.addEventListener("fetch", (event) => {
    return caches.match(event.request).then((response) => response);
});
