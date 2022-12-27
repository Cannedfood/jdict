#!/bin/bash -e

rm -Rf release || true

npm run build
(cd server && cargo build --release)

mkdir release
cp server/target/release/server release/
cp -r res release/res
cp -r dist release/public
