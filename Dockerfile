# build the webapp
FROM node:alpine
WORKDIR /web
COPY ./web /web
# install dependencies
RUN yarn
RUN yarn build

# Start with a rust alpine image
FROM rust:1-alpine3.16
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
# if needed, add additional dependencies here
RUN apk add --no-cache musl-dev
# set the workdir and copy the source into it
WORKDIR /app
COPY ./ /app
# copy the webapp build from the previous stage
COPY --from=0 /web/dist /app/dist
# do a release build
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

# use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.16
# set default env vars
ENV MITTS_ADDRESS 0.0.0.0:8000
ENV MITTS_DATABASE /config/mitts.sqlite
ENV MITTS_CONFIG /config/mitts.toml
# if needed, install additional dependencies here
RUN apk add --no-cache libgcc
# copy the binary into the final image
COPY --from=1 /app/target/release/ovenmitts .
# set the binary as entrypoint
ENTRYPOINT ["/ovenmitts"]
