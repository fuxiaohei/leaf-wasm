FROM rust as builder

WORKDIR /usr/src/leaf-runner
ADD . .

RUN rustup component add rustfmt
RUN cargo build --release

FROM debian:stable-slim

EXPOSE 19988

WORKDIR /opt/bin/

COPY --from=builder /usr/src/leaf-runner/target/release/leaf-runner /opt/bin/leaf-runner
CMD ["./leaf-runner"]