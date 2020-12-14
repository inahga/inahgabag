#!/bin/bash
set -euo pipefail

if [ -z "${1+x}" ] || [ -z "${2+x}" ]; then
	echo "usage: $0 [TARGET PACKAGE] [APPEND TO SUMMARY] [PATCH FILES]..."
	exit 1
fi

TARGET=$1
APPEND=$2
shift 2

install_deps() {
	sudo dnf install -y make rpm-build dnf-plugins-core
}

install_srpm() {
	dnf download --source "$TARGET" --destdir /tmp
	rpm -ivh "/tmp/$TARGET*"
}

cp_patches() {
	for PATCH in "$@"; do
		cp "$PATCH" "$HOME/rpmbuild/SOURCES/"
	done
}

fix_spec() {
	SPECFILE="$HOME/rpmbuild/SPECS/$TARGET.spec"
	LATEST_PATCH=$(grep -o 'Patch[0-9]\+:' "$SPECFILE" | tail -n1 || true)
	if [ -z "$LATEST_PATCH" ]; then
		PATCHNUMBER=0
	else
		PATCHNUMBER=$(($(echo "$LATEST_PATCH" | grep -o '[0-9]\+') + 1))
	fi

	for PATCH in "$@"; do
		FULLPATCH="Patch$PATCHNUMBER: $PATCH"
		if [ -z "$LATEST_PATCH" ]; then
			sed -i "/^Source0:.*/a $FULLPATCH" "$SPECFILE"
		else
			sed -i "/^$LATEST_PATCH.*/a $FULLPATCH" "$SPECFILE"
		fi

		PATCHNUMBER=$((PATCHNUMBER + 1))
		LATEST_PATCH=$FULLPATCH
	done

	sed -i "/Summary:/ s/$/ $APPEND/" "$SPECFILE"
}

build() {
	sudo dnf builddep "$TARGET" -y
	rpmbuild -ba "$HOME/rpmbuild/SPECS/$TARGET.spec"
}

install_deps
install_srpm
cp_patches "$@"
fix_spec "$@"
build
