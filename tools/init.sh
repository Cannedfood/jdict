#!/bin/bash -e

(
    echo
    echo "=============== Installing dependencies in tools/... =============="
    cd tools
    npm install
)

(
    echo
    echo "=============== Installing dependencies in code/web/... =============="
    cd code/web
    npm install
)

(
    echo
    echo "=============== Pre-Building code/server/ and code/tauri/... ==================="
    cargo build
    cargo build --release
)