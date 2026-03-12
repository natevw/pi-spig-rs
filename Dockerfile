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
#    kubectl run spig --image=localhost/spig --image-pull-policy=Never
#    kubectl logs spig

# kubectl debug -it spig --image=localhost/spig --image-pull-policy=Never --profile=sysadmin -- sh
# --see https://github.com/kubernetes/kubectl/issues/1781#issuecomment-3287214675
# kubectl debug -it spig --image=localhost/spig --image-pull-policy=Never --custom asRoot.yaml -- sh
#    # HT: https://unix.stackexchange.com/questions/18061/why-does-sh-say-not-found-when-its-definitely-there/18079#18079
#    # and https://stackoverflow.com/questions/47144933/no-such-file-or-directory-error-when-running-a-dynamically-linked-arm-executab
#    apk add binutils
#    readelf -l /app/pi-spig-rs   # [Requesting program interpreter: /lib/ld-linux-armhf.so.3]
#    readelf -l /bin/ls           # [Requesting program interpreter: /lib/ld-musl-armhf.so.1]
#    ln -sf /lib/ld-musl-armhf.so.1 /lib/ld-linux-armhf.so.3  # HT: https://stackoverflow.com/questions/77850936/how-to-specify-the-default-dynamic-linker-name-when-building-gcc-glibc/77852782#77852782
#       ----> Error loading shared library libgcc_s.so.1: No such file or directory (needed by /app/pi-spig-rs)
# https://unix.stackexchange.com/questions/18061/why-does-sh-say-not-found-when-its-definitely-there

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


FROM docker.io/library/alpine:3 AS final
#FROM scratch AS final    # also works, just comment out `adduser` below!
COPY --from=build /app/.install/bin/ /app/
ARG UID=10001
RUN adduser --disabled-password --no-create-home --uid "${UID}" appuser
USER $UID
EXPOSE 8000
ENTRYPOINT ["/app/pi-spig-rs"]
CMD ["100", "2"]
