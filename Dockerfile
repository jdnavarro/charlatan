FROM rust:1.43.0 as build-env
WORKDIR /app
ADD . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/charlatan-server /
CMD ["./charlatan-server"]
