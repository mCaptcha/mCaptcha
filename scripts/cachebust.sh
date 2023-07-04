#!/bin/bash

# SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
#
# SPDX-License-Identifier: AGPL-3.0-or-later

set -Eeuo pipefail
trap cleanup SIGINT SIGTERM ERR EXIT

readonly PROJECT_ROOT=$(realpath $(dirname $(dirname "${BASH_SOURCE[0]}")))
source $PROJECT_ROOT/scripts/lib.sh

readonly DIST=$PROJECT_ROOT/static/cache/bundle/


file_extension() {
	echo $1 | rev | tr
}

cache_bust(){
	name=$(get_file_name $1)
	extension="${name##*.}"
	filename="${name%.*}"
	file_hash=$(sha256sum $1 | cut -d " " -f 1 | tr "[:lower:]" "[:upper:]") 

	msg "${GREEN}- Processing $name: $filename.$file_hash.$extension"

	sed -i \
		"s/$name/assets\/bundle\/$filename.$file_hash.$extension/" \
		$(find $DIST -type f -a -name "*.js")
}

setup_colors

msg "${BLUE}[*] Setting up files for cache busting"

for file in $(find $DIST  -type f -a -name "*.js")
do
	name=$(get_file_name $file)
	case $name in
		"bench.js")
			cache_bust $file
			;;
	esac
done
