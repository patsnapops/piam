#!/bin/bash

RUSTFLAGS="--cfg unsound_local_offset" cargo watch -x 'run --features s3' -d1
