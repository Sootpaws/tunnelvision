FROM rust:latest as build
# Build server
WORKDIR /build
COPY src src
COPY static static
COPY Cargo.* .
RUN cargo build --release
# Fetch dataset
RUN apt-get update && apt-get install rclone
RUN --mount=type=secret,id=rclone_config \
    rclone --progress --config=/run/secrets/rclone_config copy source: /data

# Build final image
FROM scratch
# System files
COPY --from=build /lib/x86_64-linux-gnu /lib/x86_64-linux-gnu
COPY --from=build /lib64 /lib64
COPY --from=build /bin/sh /bin/sh
# App files
COPY --from=build /build/target/release/tunnelvision /tunnelvision
COPY --from=build /build/static /static
COPY --from=build /data /data
# Validate dataset
RUN /tunnelvision \
    --validate-only --data-path /data --image-cache /tmp/image_cache
# Run server
EXPOSE 8080
CMD [ "/tunnelvision", \
    "--data-path", "/data", "--image-cache", "/tmp/image_cache" ]
