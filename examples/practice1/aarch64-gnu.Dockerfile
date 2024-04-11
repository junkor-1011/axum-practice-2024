FROM gcr.io/distroless/cc-debian12:nonroot as ref

COPY target/aarch64-unknown-linux-gnu/release/practice1 /app/app

EXPOSE 3000
WORKDIR /app
USER nonroot

ENTRYPOINT ["./app"]
