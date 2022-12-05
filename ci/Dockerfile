# base:rust-musl-alpine3.16
#FROM rust:alpine3.16
#RUN apk add  \
#    musl-dev  \
#    pkgconfig  \
#    openssl-dev

# base:cargo-cached
FROM 955466075186.dkr.ecr.cn-northwest-1.amazonaws.com.cn/ops-basic/base:rust-musl-alpine3.16
RUN mkdir ~/.cargo
RUN echo $'[source.crates-io] \n\
replace-with = \'sjtu\' \n\
[source.sjtu] \n\
registry = \"https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"' > ~/.cargo/config.toml
RUN cargo search --limit 0
