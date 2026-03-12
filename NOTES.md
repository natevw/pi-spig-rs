

## Container claiming "not found"

Getting `sh: /app/pi-spig-rs: not found` when it's there and +x set!

```
kubectl debug -it spig --image=localhost/spig --image-pull-policy=Never --profile=sysadmin -- sh
# doesn't help……


vi asRoot.yaml

  # via https://github.com/kubernetes/kubectl/issues/1781#issuecomment-3287214675
  securityContext:
    runAsUser: 0
    runAsGroup: 0

kubectl debug -it spig --image=localhost/spig --image-pull-policy=Never --custom asRoot.yaml -- sh
```

HT: https://unix.stackexchange.com/questions/18061/why-does-sh-say-not-found-when-its-definitely-there/18079#18079 and https://stackoverflow.com/questions/47144933/no-such-file-or-directory-error-when-running-a-dynamically-linked-arm-executab

In container:

```
apk add binutils
readelf -l /app/pi-spig-rs   # [Requesting program interpreter: /lib/ld-linux-armhf.so.3]
readelf -l /bin/ls           # [Requesting program interpreter: /lib/ld-musl-armhf.so.1]
ln -sf /lib/ld-musl-armhf.so.1 /lib/ld-linux-armhf.so.3  # HT: https://stackoverflow.com/questions/77850936/how-to-specify-the-default-dynamic-linker-name-when-building-gcc-glibc/77852782#77852782
  ----> Error loading shared library libgcc_s.so.1: No such file or directory (needed by /app/pi-spig-rs)
```