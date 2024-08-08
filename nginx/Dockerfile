# Copyright © 2024 Akira Miyakoda
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

FROM node:20-bookworm AS builder

WORKDIR /tmp/app

COPY ./pwa/package*.json .
RUN npm install

COPY ./pwa .
RUN npm run build

FROM nginx:stable

RUN rm -rf /usr/share/nginx/html/*
COPY --from=builder /tmp/app/dist /usr/share/nginx/html
