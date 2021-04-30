# WIP
default: build-frontend
	cargo build

run: build-frontend
	cargo run

dev-env:
	cargo fetch
	yarn install

docs:
	cargo doc --no-deps --workspace --all-features

build-frontend-dev:
	yarn start

build-frontend:
	yarn build

test: migrate
	cargo test

xml-test-coverage: migrate
	cargo tarpaulin -t 1200 --out Xml

coverage: migrate
	cargo tarpaulin -t 1200 --out Html

release: build-frontend
	cargo build --release

clean:
	cargo clean
	yarn clean

migrate:
	cargo run --bin tests-migrate

help:
	@echo  '  docs      	- build documentation'
	@echo  '  run       	- run developer instance'
	@echo  '  test 		- run unit and integration tests'
	@echo  '  migrate   	- run database migrations'
	@echo  '  dev-env 	- download dependencies'
	@echo  '  clean     	- drop builds and environments'
	@echo  '  coverage 	- build test coverage in HTML format'
	@echo  '  xml-coverage 	- build test coverage in XML for upload to codecov'
	@echo  ''
