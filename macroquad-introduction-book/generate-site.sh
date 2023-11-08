#!/usr/bin/env sh

SITE_PATH="../../macroquad-introduction-book-site/"

mdbook build
cp -pR book/* "$SITE_PATH"
cd "$SITE_PATH"
git commit -am "Update site"
git push
cd -
