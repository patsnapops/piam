#!/bin/bash

docker build -t 955466075186.dkr.ecr.cn-northwest-1.amazonaws.com.cn/ops-basic/base:cargo-cached ./
docker push 955466075186.dkr.ecr.cn-northwest-1.amazonaws.com.cn/ops-basic/base:cargo-cached
