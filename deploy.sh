#! /usr/bin/env bash

set -e

cargo web deploy -p slides_frontend --target wasm32-unknown-unknown --release

rm -rf ./gh-pages
git clone -b gh-pages git@github.com:edvorg/slides.rs.git ./gh-pages

pushd ./gh-pages

rm -f ./*.css
rm -f ./*.html
rm -f ./*.js
rm -f ./*.wasm
rm -f ./*.png
mv -f ../target/deploy/* ./

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
