all:
	nix-build '<nixpkgs/nixos>' -A vm -I nixos-config=configuration.nix

run:
	result/bin/run-nixos-vm

.PHONY: all run
