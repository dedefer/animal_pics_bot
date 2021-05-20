FROM ekidd/rust-musl-builder:latest as base

COPY . /home/rust/src

RUN cargo build --release

RUN strip /home/rust/src/target/x86_64-unknown-linux-musl/release/cats_pic_bot

RUN chmod +x /home/rust/src/target/x86_64-unknown-linux-musl/release/cats_pic_bot

FROM scratch

COPY --from=base /home/rust/src/target/x86_64-unknown-linux-musl/release/cats_pic_bot /cats_pic_bot

CMD ["/cats_pic_bot"]
