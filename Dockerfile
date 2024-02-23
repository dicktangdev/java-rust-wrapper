FROM rust:1.67 as builder
ARG LITCRYPT_ENCRYPT_KEY

# Set the build argument as an environment variable
ENV LITCRYPT_ENCRYPT_KEY=${LITCRYPT_ENCRYPT_KEY}

WORKDIR /usr/src/rust-jar-wrapper
COPY . .
# RUN cargo build
RUN cargo install --path .

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y openjdk-8-jre && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY xxx.jar .
COPY --from=builder /usr/local/cargo/bin/rust-jar-wrapper /usr/local/bin/rust-jar-wrapper

CMD ["rust-jar-wrapper", "xxx.jar"]

# CMD tail -f /dev/null


# docker build --build-arg LITCRYPT_ENCRYPT_KEY=xxxx -t xxx .
