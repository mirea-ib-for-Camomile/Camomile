FROM rust:1.64-alpine3.16
COPY . .
RUN cargo build --release
EXPOSE 7878
CMD ["./target/release/camomile"]
#FIXME: not run, but build