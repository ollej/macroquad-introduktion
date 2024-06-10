#!/usr/bin/env sh

SITE_PATH="../../macroquad-introduction-book-english-site/"

mdbook build
cp -pR book/html/* "$SITE_PATH"
cp book/pdf/output.pdf "$SITE_PATH/pdf/"
cd "$SITE_PATH"
git add -A
git commit -m "Update site"
git push
cd -
