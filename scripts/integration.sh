#!/bin/bash

set -Eeuo pipefail
trap cleanup SIGINT SIGTERM ERR EXIT

readonly PROJECT_ROOT=$(realpath $(dirname $(dirname "${BASH_SOURCE[0]}")))

source $PROJECT_ROOT/scripts/lib.sh



docker-compose down -v --remove-orphans || true
docker-compose up -d
cd $(mktemp -d)
git clone https://github.com/mCaptcha/integration . 
yarn install
npx nightwatch ./test/mCaptcha.ts
cd $PROJECT_ROOT
docker-compose down -v --remove-orphans || true
