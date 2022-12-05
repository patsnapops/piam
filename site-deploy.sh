#!/bin/bash

# !! This file is for development purpose only, DO NOT USE if you do not know what it does !!

# deploy site

cd website || exit
pnpm build

## scp static site artifacts
ssh root@ida.patsnap.info "cd ~/d/site/piam/; rm -rf ./*; pwd;"
cd ./build || exit
scp -pr * "root@ida.patsnap.info:~/d/site/piam"
