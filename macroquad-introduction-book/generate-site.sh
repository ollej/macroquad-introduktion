#!/usr/bin/env sh

mdbook build
cp -pR book/* ../../macroquad-introduction-book-site/
