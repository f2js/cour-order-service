FROM rust:1.65 AS builder

COPY . .
RUN cargo build --release

FROM debian:stable-slim
RUN apt-get update
RUN apt-get install -y openssl

EXPOSE 8080

COPY --from=builder ./target/release/cour_order_service ./target/release/cour_order_service
CMD ["./target/release/cour_order_service"]