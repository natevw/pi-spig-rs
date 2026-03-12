# HT: https://docs.docker.com/guides/rust/build-images/

# USAGE:
# sudo apt install podman crun  # via https://wiki.debian.org/Podman
# systemctl --user start dbus   # via https://github.com/containers/podman/issues/12983#issuecomment-1320376753
# podman build .


ARG RUST_VERSION=1.94
ARG APP_NAME=pi-spig-rs

FROM docker.io/library/rust:${RUST_VERSION}-alpine AS build
RUN apk add --no-cache clang lld musl-dev git
ARG APP_NAME
WORKDIR /app
RUN --mount=type=bind,source=src,target=/app/src \
    --mount=type=bind,source=Cargo.toml,target=/app/Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=/app/Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server


FROM docker.io/library/alpine:3.18 AS final
COPY --from=build /bin/server /bin/
ARG UID=10001
RUN adduser --disabled-password --no-create-home --uid "${UID}" appuser
USER appuser
EXPOSE 8000
CMD ["/bin/server"]
