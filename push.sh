#!/usr/bin/env bash

git add .
git commit -m $0
git push origin main

cp public/* ../t56k.github.io
cd ../t56k.github.io
git add .
git commit -m $0
git push origin main

cd ../ssrg
