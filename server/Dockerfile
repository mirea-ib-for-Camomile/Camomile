FROM rust:1.65-alpine3.16
COPY . .
RUN cargo build --release
EXPOSE 7878
CMD ["./target/release/server"]