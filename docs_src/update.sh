#!/usr/bin/env sh

mdbook build
rm -rf ../docs
mkdir ../docs
cp -r book/* ../docs
