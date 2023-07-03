#!/bin/bash

set -Eeuo pipefail
trap cleanup SIGINT SIGTERM ERR EXIT

readonly PROJECT_ROOT=$(realpath $(dirname $(dirname "${BASH_SOURCE[0]}")))

source $PROJECT_ROOT/scripts/lib.sh

is_ci(){
    if [ -z ${CI+x} ];
    then
        return 1
    else
        return 0
    fi
}



docker-compose down -v --remove-orphans || true
docker-compose up -d
cd $(mktemp -d)
pwd
find 
git clone https://github.com/mCaptcha/integration . 

if is_ci
then
	yarn install
	xvfb-run --auto-servernum npm run test.firefox
	xvfb-run --auto-servernum npm run test.chrome
else
	yarn install
	npx nightwatch ./test/mCaptcha.ts
fi

cd $PROJECT_ROOT
docker-compose down -v --remove-orphans || true
