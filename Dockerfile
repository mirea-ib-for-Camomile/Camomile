FROM rust:1.64
COPY ./ ./
RUN cargo build --release
CMD ["./target/release/holodeck"]
#FIXME: not run, but build