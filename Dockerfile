# Start with a rust alpine image
FROM rust:1-alpine3.16
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
# if needed, add additional dependencies here
RUN apk add --no-cache musl-dev
# set the workdir and copy the source into it
WORKDIR /app
COPY ./ /app
# do a release build
RUN cargo build --release

# use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.16
# set default env vars
ENV MITTS_ADDRESS 0.0.0.0
ENV MITTS_DATABASES {mitts={url="/config/mitts.sqlite"}}
ENV MITTS_CONFIG /config/mitts.toml
# if needed, install additional dependencies here
RUN apk add --no-cache libgcc
# copy the binary into the final image
COPY --from=0 /app/target/release/ovenmitts .
# set the binary as entrypoint
ENTRYPOINT ["/ovenmitts"]
