#!/usr/bin/env sh

mdbook build
rm -rf ../www
mkdir ../www
cp -r book/* ../www
