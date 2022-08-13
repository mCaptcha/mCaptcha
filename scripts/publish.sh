#!/bin/bash

# publish.sh: grab bin from docker container, pack, sign and upload
# $1: mCaptcha version
# $2: Docker img tag
set -xEeuo  pipefail

TMP_DIR=$(mktemp -d)
FILENAME="mCaptcha-$1-linux-amd64"
TARBALL="mCaptcha-$1.tar.gz"
TARGET_DIR="$TMP_DIR/$FILENAME"
DOCKER_IMG="mcaptcha/mcaptcha:$2"

mkdir $TARGET_DIR

get_bin(){
	echo "[*] Grabbing binary"
	container_id=$(docker create $DOCKER_IMG)
	#docker cp $container_id:/usr/local/bin/mcaptcha $TARGET_DIR/
	docker cp $container_id:/usr/local/bin/mcaptcha $TARGET_DIR/
	docker rm -v $container_id
}

copy() {
	echo "[*] Copying dist assets"
	cp README.md  $TARGET_DIR
	cp LICENSE.md $TARGET_DIR
	cp CHANGELOG.md $TARGET_DIR
	cp docker-compose.yml $TARGET_DIR

	mkdir $TARGET_DIR/docs
	cp docs/DEPLOYMENT.md $TARGET_DIR/docs
	cp docs/CONFIGURATION.md $TARGET_DIR/docs

	get_bin
}

pack() {
	echo "[*] Creating dist tarball"
	tar -cvzf $TARBALL $TARGET_DIR 
}

checksum() {
	echo "[*] Generating dist tarball checksum"
	sha256sum $TARBALL > $TARBALL.sha256
}

sign() {
	echo "[*] Signing dist tarball checksum"
	gpg --output $TARBALL.asc --sign --detach --armor $TARBALL
}

copy
pushd $TMP_DIR
pack
checksum
sign
popd
