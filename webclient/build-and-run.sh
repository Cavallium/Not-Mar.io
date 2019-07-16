#!/usr/bin/env bash
wasm-pack build
cd www
npm install --reinstall not_mario_webclient
npm run build
npm run start
open "http://localhost:8080"