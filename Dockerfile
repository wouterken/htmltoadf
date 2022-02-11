FROM rust:1.58-buster as builder

WORKDIR /usr/src/html2adf
COPY . .

RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/html2adf /usr/local/bin/html2adf

ENTRYPOINT ["html2adf"]
CMD []
