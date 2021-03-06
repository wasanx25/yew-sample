FROM rust:1.50.0

ENV TRUNK_VERSION=v0.8.3
WORKDIR /var/local

RUN apt-get -y update && \
        wget -qO- https://github.com/thedodd/trunk/releases/download/${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- && \
        mv trunk /usr/local/bin/ && \
        cargo install wasm-bindgen-cli && \
        rustup target add wasm32-unknown-unknown

CMD ["trunk", "build"]
