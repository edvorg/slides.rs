#! /usr/bin/env bash

set -e

rm -rf ./gh-pages
git clone -b gh-pages git@github.com:edvorg/slides.rs.git ./gh-pages

DEPLOY_DIR=$(realpath ./target/deploy)

function build {
    echo $DEPLOY_DIR $1 $2

    rm -rf $DEPLOY_DIR/*
    cargo web deploy -p $2 --target wasm32-unknown-unknown --release

    mkdir -p $1

    rm -f $1/*.css
    rm -f $1/*.html
    rm -f $1/*.js
    rm -f $1/*.wasm
    rm -f $1/*.png
    mv -f $DEPLOY_DIR/* $1

    pushd $1

    CSS_CHECKSUM=$(md5sum ./index.css | awk '{print $1}' | xargs echo -n)
    FRONTEND_CHECKSUM=$(md5sum ./$2.wasm | awk '{print $1}' | xargs echo -n)

    sed -i "s/index.css/index.css?hash=${CSS_CHECKSUM}/g" ./index.html
    sed -i "s/$2.js/$2.js?hash=${FRONTEND_CHECKSUM}/g" ./index.html
    sed -i "s/$2.wasm/$2.wasm?hash=${FRONTEND_CHECKSUM}/g" ./$2.js

    git add *.css
    git add *.html
    git add *.js
    git add *.wasm
    git add *.png

    popd
}

build $(realpath ./gh-pages) slides_frontend
build $(realpath ./gh-pages/talk-1) 01_full_stack_web_development

pushd ./gh-pages

git commit -m "update"
git push origin gh-pages

popd
