# HT: https://docs.docker.com/guides/rust/build-images/

# USAGE:
# sudo apt install podman crun  # via https://wiki.debian.org/Podman
# systemctl --user start dbus   # via https://github.com/containers/podman/issues/12983#issuecomment-1320376753
# podman build -t spig .
# podman run spig


FROM docker.io/library/rust:1-alpine AS build
RUN apk add --no-cache clang lld musl-dev git
WORKDIR /app
RUN --mount=type=bind,source=src,target=/app/src \
    --mount=type=bind,source=Cargo.toml,target=/app/Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=/app/Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    mkdir .install && cargo install --locked --root=.install --path=.


FROM docker.io/library/alpine:3 AS final
COPY --from=build /app/.install/bin/ /app/
ARG UID=10001
RUN adduser --disabled-password --no-create-home --uid "${UID}" appuser
USER appuser
EXPOSE 8000
ENTRYPOINT ["/app/pi-spig-rs"]
CMD ["100", "2"]
