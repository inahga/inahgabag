all: build run

build:
	rm *.qcow2
	nix-build '<nixpkgs/nixos>' -A vm -I nixos-config=configuration.nix

run:
	QEMU_OPTS="-vga cirrus -m 8192 -smp 4" result/bin/run-*-vm

.PHONY: all run build
