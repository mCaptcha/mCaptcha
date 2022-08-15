#!/bin/bash
# Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
# 
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
# 
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
# 
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# publish.sh: grab bin from docker container, pack, sign and upload
# $2: binary version
# $3: Docker img tag
# $4: dumbserve password

set -xEeuo  pipefail

DUMBSERVE_USERNAME=mcaptcha
DUMBSERVE_PASSWORD=$4
DUMBSERVE_HOST="https://$DUMBSERVE_USERNAME:$DUMBSERVE_PASSWORD@dl.mcaptcha.org"

NAME=mcaptcha
KEY=0CBABF3084E84E867A76709750BE39D10ECE01FB

TMP_DIR=$(mktemp -d)
FILENAME="$NAME-$2-linux-amd64"
TARBALL=$FILENAME.tar.gz
TARGET_DIR="$TMP_DIR/$FILENAME/"
mkdir -p $TARGET_DIR
DOCKER_IMG="mcaptcha/$NAME:$3"


get_bin(){
	echo "[*] Grabbing binary"
	container_id=$(docker create $DOCKER_IMG)
	docker cp $container_id:/usr/local/bin/$NAME $TARGET_DIR/
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
	pushd $TMP_DIR
	tar -cvzf $TARBALL $FILENAME
	popd
}

checksum() {
	echo "[*] Generating dist tarball checksum"
	pushd $TMP_DIR
	sha256sum $TARBALL > $TARBALL.sha256
	popd
}

sign() {
	echo "[*] Signing dist tarball checksum"
	pushd $TMP_DIR
	export GPG_TTY=$(tty)
	gpg --verbose \
		--pinentry-mode loopback \
		--batch --yes \
		--passphrase $GPG_PASSWORD \
		--local-user $KEY \
		--output $TARBALL.asc \
		--sign --detach \
		--armor $TARBALL
	popd
}

delete_dir() {
	curl --location --request DELETE "$DUMBSERVE_HOST/api/v1/files/delete" \
		--header 'Content-Type: application/json' \
		--data-raw "{
			\"path\": \"$1\"
		}"
}

upload_dist() {
	delete_dir $1

	pushd $TMP_DIR
	for file in $TARBALL $TARBALL.asc $TARBALL.sha256
	do
		curl -v \
			-F upload=@$file  \
			"$DUMBSERVE_HOST/api/v1/files/upload?path=mCaptcha/$1/"
	done
	popd
}

publish() {
	copy
	pack
	checksum
	sign
	upload_dist $2
}

$1 $@
