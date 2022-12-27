#!/bin/bash -e

(
    echo
    echo "=============== Installing dependencies in tools/... =============="
    cd tools
    npm install
)

(
    echo
    echo "=============== Installing dependencies in web/... =============="
    cd web
    npm install
)

(
    echo
    echo "=============== Pre-Building server/... ==================="
    cd server
    cargo build
    cargo build --release
)