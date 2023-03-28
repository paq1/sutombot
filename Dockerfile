FROM rust:latest
COPY . .
WORKDIR /
RUN cargo build --release
# pas besoin de l'exposer EXPOSE 8000
CMD ["./target/release/sutombot"]