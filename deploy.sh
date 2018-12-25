#! /usr/bin/env bash

set -e

cargo clean
cargo web deploy -p slides_frontend --target wasm32-unknown-unknown --release

rm -rf ./gh-pages
git clone -b gh-pages . ./gh-pages

rm -f ./gh-pages/*.css
rm -f ./gh-pages/*.html
rm -f ./gh-pages/*.js
rm -f ./gh-pages/*.wasm
rm -f ./gh-pages/*.png
mv -f ./target/deploy/* ./gh-pages/

pushd ./gh-pages

CSS_CHECKSUM=$(md5sum ./index.css | awk '{print $1}' | xargs echo -n)
FRONTEND_CHECKSUM=$(md5sum ./slides_frontend.wasm | awk '{print $1}' | xargs echo -n)

sed -i "s/index.css/index.css?hash=${CSS_CHECKSUM}/g" ./index.html
sed -i "s/slides_frontend.js/slides_frontend.js?hash=${FRONTEND_CHECKSUM}/g" ./index.html
sed -i "s/slides_frontend.wasm/slides_frontend.wasm?hash=${FRONTEND_CHECKSUM}/g" ./slides_frontend.js

git add *.css
git add *.html
git add *.js
git add *.wasm
git add *.png

git commit -m "update"
git push origin gh-pages

popd

git push origin gh-pages
