FROM gcr.io/distroless/static-debian12:nonroot as ref

FROM scratch as final

COPY --from=ref /etc/passwd /etc/
COPY --from=ref /etc/group /etc/
COPY target/x86_64-unknown-linux-musl/release/practice1 /app/app

EXPOSE 3000
WORKDIR /app
USER nonroot

CMD ["./app"]
