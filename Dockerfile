FROM rust:latest as builder
WORKDIR /app/
COPY . .
RUN cargo b --release

FROM ubuntu:latest  
WORKDIR /app/
COPY --from=builder /app/target/release/warp_docker /usr/local/bin
