# base:rust-musl-alpine3.16
#FROM rust:alpine3.16
#RUN apk add  \
#    musl-dev  \
#    pkgconfig  \
#    openssl-dev

# base:cargo-cached
#FROM 955466075186.dkr.ecr.cn-northwest-1.amazonaws.com.cn/ops-basic/base:rust-musl-alpine3.16
#RUN mkdir ~/.cargo
#RUN echo $'[source.crates-io] \n\
#replace-with = \'sjtu\' \n\
#[source.sjtu] \n\
#registry = \"https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"' > ~/.cargo/config.toml
#RUN cargo search --limit 0

# builder stage
FROM 955466075186.dkr.ecr.cn-northwest-1.amazonaws.com.cn/ops-basic/base:cargo-cached AS builder
#// TODO: push builder-cached-${package} to ecr for each build, use it next time
#FROM builder-cached AS builder
ARG package
RUN echo "package to build: ${package}"
COPY . .
RUN rustc --version
RUN cargo --version
RUN cargo build --profile=release-with-debug --all-features -p ${package}
RUN ls -lh ./target/release-with-debug

# runtime stage
#FROM alpine:3.16 as runtime
FROM 955466075186.dkr.ecr.cn-northwest-1.amazonaws.com.cn/ops-basic/base:alpine3.16-tool AS runtime
ARG package
ENV ENV_PACKAGE=${package}
RUN echo "package to run: ${package}"
COPY --from=builder ./target/release-with-debug/${package} /opt/${package}
RUN ls /opt/
WORKDIR /opt/logs/apps/
CMD ["/bin/sh", "-c", "/opt/${ENV_PACKAGE}"]
