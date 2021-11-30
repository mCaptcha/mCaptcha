#!/bin/bash

readonly DIST=static/cache/bundle/

readonly LICENSE_START="// @license magnet:?xt=urn:btih:0b31508aeb0634b347b8270c7bee4d411b5d4109&dn=agpl-3.0.txt AGPL-3.0"
readonly SOURCE="// @source https://github.com/mCaptcha/mCaptcha"
readonly LICENSE_END="// @license-end"
echo $LICENSE_START
echo $LICENSE_END


for file in $(find ./static/cache/bundle/  -type f -a -name "*.js")
do
	contents=$(cat $file)
	echo $LICENSE_START > $file
	echo $SOURCE >> $file
	echo $contents >> $file
	echo $LICENSE_END >> $file
done
