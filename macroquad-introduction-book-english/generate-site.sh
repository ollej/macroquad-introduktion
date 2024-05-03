#!/usr/bin/env sh

SITE_PATH="../../macroquad-introduction-book-english-site/"

mdbook build
cp -pR book/* "$SITE_PATH"
cd "$SITE_PATH"
git add -A
git commit -m "Update site"
git push
cd -
