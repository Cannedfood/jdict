#!/bin/bash -e

(
    echo
    echo "=============== Installing dependencies in tools/... (for generating image sizes) =============="
    cd tools
    npm install
)

(
    echo
    echo "=============== Generating various image sizes in code/web/public... =============="
    ./tools/generate-sizes.mjs
)

(
    echo
    echo "=============== Installing dependencies in code/web/... =============="
    cd code/web
    npm install
)

(
    echo
    echo "=============== Prebuilding code/web/... =============="
    cd code/web
    npm run build
)

(
    echo
    echo "=============== Pre-Building rust code... ==================="
    cargo build
    cargo build --release
)
