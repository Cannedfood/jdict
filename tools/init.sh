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
    echo "=============== Generating various image sizes in code/web/public... =============="
    ./tools/generate-sizes.mjs
)

(
    echo
    echo "=============== Pre-Building rust code... ==================="
    cargo build
    cargo build --release
)
