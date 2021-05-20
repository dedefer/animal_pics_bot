FROM ekidd/rust-musl-builder:latest as base

COPY . /home/rust/src

RUN cargo build --release

RUN strip /home/rust/src/target/x86_64-unknown-linux-musl/release/cats_pic_bot

RUN chmod +x /home/rust/src/target/x86_64-unknown-linux-musl/release/cats_pic_bot

FROM alpine:latest

COPY --from=base /home/rust/src/target/x86_64-unknown-linux-musl/release/cats_pic_bot /app/cats_pic_bot

CMD /app/cats_pic_bot
