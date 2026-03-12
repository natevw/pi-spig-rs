# HT: https://docs.docker.com/guides/rust/build-images/

# USAGE:
# sudo apt install podman crun  # via https://wiki.debian.org/Podman
# systemctl --user start dbus   # via https://github.com/containers/podman/issues/12983#issuecomment-1320376753

# sudo apt install qemu-user
# sudo podman run --privileged --rm docker.io/tonistiigi/binfmt --install all
# podman build --platform=linux/arm/7 -t spig .
#    podman run spig
# podman save spig > spig-img.tar
# scp spig-img.tar pi@turing-node-0.lan:
#  sudo k3s ctr images import spig-img.tar
#    sudo k3s crictl images
#    kubectl delete pod spig
#    kubectl run spig --image=localhost/spig --image-pull-policy=Never --restart=OnFailure
#    kubectl get pods
#    kubectl logs spig


FROM docker.io/library/rust:1-slim-trixie AS build
#RUN apk add --no-cache clang lld musl-dev git
WORKDIR /app
RUN --mount=type=bind,source=src,target=/app/src \
    --mount=type=bind,source=Cargo.toml,target=/app/Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=/app/Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    mkdir .install && cargo install --locked --root=.install --path=.


FROM docker.io/library/debian:trixie-slim AS final
COPY --from=build /app/.install/bin/ /app/
ARG UID=10001
RUN useradd --no-create-home --uid "${UID}" appuser
USER appuser
EXPOSE 8000
ENTRYPOINT ["/app/pi-spig-rs"]
CMD ["100", "2"]
