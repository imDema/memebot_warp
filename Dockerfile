FROM rust:1.41-slim as builder

RUN apt-get update && apt-get install -y \
 libssl-dev\
 pkg-config\
 gcc-multilib\
 libpq-dev

WORKDIR /usr/src/memebot_warp
COPY . .
RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
 libssl-dev\
 pkg-config\
 gcc-multilib\
 libpq-dev\
 ca-certificates

COPY --from=builder /usr/local/cargo/bin/memebot_warp /usr/local/bin/memebot_warp
CMD [ "memebot_warp" ]
