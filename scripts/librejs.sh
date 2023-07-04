#!/bin/bash

# SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
#
# SPDX-License-Identifier: AGPL-3.0-or-later

set -Eeuo pipefail
trap cleanup SIGINT SIGTERM ERR EXIT

readonly PROJECT_ROOT=$(realpath $(dirname $(dirname "${BASH_SOURCE[0]}")))
readonly DIST=$PROJECT_ROOT/static/cache/bundle/
readonly SOURCE="// @source https://github.com/mCaptcha/mCaptcha"
readonly LICENSE_END="// @license-end"

source $PROJECT_ROOT/scripts/lib.sh

print_license_msg() {
	msg "${GREEN}- Applying $1 on  $(get_file_name $2)"
}

apply_agpl() {
	print_license_msg "AGPL" $1
	local AGPL='// @license magnet:?xt=urn:btih:0b31508aeb0634b347b8270c7bee4d411b5d4109&dn=agpl-3.0.txt AGPL-3.0'
	echo $AGPL >> $1
}

apply_x11() {
	print_license_msg "X11" $1
	local MIT='// @license magnet:?xt=urn:btih:5305d91886084f776adcf57509a648432709a7c7&dn=x11.txt X11'
	echo $MIT >> $1
}

apply_apache() {
	print_license_msg "APACHE" $1
	local APACHE='// @license magnet:?xt=urn:btih:8e4f440f4c65981c5bf93c76d35135ba5064d8b7&dn=apache-2.0.txt Apache-2.0'
	echo $APACHE >> $1
}

setup_colors

msg "${BLUE}[*] LibreJS processor running"

for file in $(find $DIST  -type f -a -name "*.js")
do
	contents=""
	cp $file $file.librejs
	: > $file

	name=$(get_file_name $file)
	case $name in
		"bundle.js")
			apply_agpl $file
			;;
		"verificationWidget.js" | "bench.js")
			apply_x11 $file
			apply_apache $file
			;;
		*)
			msg "${RED}- [!] License not configured for $name. Applying default license"
			apply_agpl $file
			;;
	esac

	echo $SOURCE >> $file
	cat $file.librejs >> $file
	rm $file.librejs
	echo $LICENSE_END >> $file
done
