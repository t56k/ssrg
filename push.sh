#!/usr/bin/env bash

git add .
git commit -m "$1"
git push origin main

cp -r public/* ../t56k.github.io
cd ../t56k.github.io
git add .
git commit -m "$1"
git push origin main

cd ../ssrg
