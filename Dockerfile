# builder images
FROM rust:alpine3.16 AS builder

COPY . .
RUN apk add  \
    musl-dev  \
    pkgconfig  \
    openssl-dev
RUN cargo update

FROM builder AS builder-manager
RUN cargo build --release -p piam-manager

FROM builder AS builder-s3-proxy
RUN cargo build --release -p piam-s3-proxy

# piam-manager image
FROM alpine:3.16 as piam-manager

COPY --from=builder-manager ./target/release/piam-manager piam-manager

CMD ["./piam-manager"]

# piam-s3-proxy image
FROM alpine:3.16 as piam-s3-proxy

COPY --from=builder-s3-proxy ./target/release/piam-s3-proxy piam-s3-proxy
CMD ["./piam-s3-proxy"]
