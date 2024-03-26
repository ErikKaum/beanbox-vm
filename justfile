distro := "ubuntu"

build: rootfs build-gotty build-cli

rootfs:
	mkdir -p rootfs
	podman create --name libkrun_chroot_vm {{ distro }}
	podman export libkrun_chroot_vm | tar xpf - -C rootfs
	podman rm libkrun_chroot_vm
	echo "options use-vc \nnameserver 1.1.1.1" > rootfs/etc/resolv.conf 

build-gotty:
    cd gotty && env GOOS=linux GOARCH=arm64 make gotty && mv gotty ../rootfs/bin/gotty

build-cli:
	cd cli && cargo build --release && mv target/release/beanbox ../beanbox
	codesign --entitlements cli/cli.entitlements -s - beanbox

