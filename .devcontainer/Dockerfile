FROM rust:1.47.0-slim

RUN apt update \
    && apt install -y --no-install-recommends \
    apt-utils \
    git \
    ca-certificates \
    && apt autoremove -y \
    && apt clean -y \
    && rm -rf /var/lib/apt/lists/* \
    && rustup component add rust-analysis --toolchain 1.47.0-x86_64-unknown-linux-gnu
    && rustup component add rust-src --toolchain 1.47.0-x86_64-unknown-linux-gnu
    && rustup component add rls --toolchain 1.47.0-x86_64-unknown-linux-gnu
