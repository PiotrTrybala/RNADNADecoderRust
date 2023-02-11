FROM rust:1.67

WORKDIR /usr/src/decoder/
COPY . .

RUN rustup default nightly
RUN cargo install --path .
RUN cargo build

CMD ["./target/debug/Decoder"]

