FROM rust:1.64.0 as builder
WORKDIR /usr/src/bobashare
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
# RUN apt-get update \
#     && apt-get install extra-runtime-dependencies \
#     && rm -rf /var/lib/apt/lists/* \
#     ;
COPY --from=builder /usr/src/bobashare/target/release/bobashare-web /usr/local/bin/bobashare-web
COPY --from=builder /usr/src/bobashare/target/release/bobashare-admin /usr/local/bin/bobashare-admin
ENV APP_LISTEN_ADDR="0.0.0.0:3000"
ENV APP_BACKEND_PATH="/data"
EXPOSE 3000/tcp
CMD ["bobashare-web"]
