default: frontend
	cargo build

run: frontend
	cargo run

dev-env:
	cargo fetch
	yarn install

doc:
	#yarn doc
	cargo doc --no-deps --workspace --all-features
	cd browser && cargo doc --no-deps --workspace --all-features

frontend:
	cd browser && wasm-pack build --release
	yarn install
	yarn build

test: migrations
	cd browser && wasm-pack test --release --headless --chrome
	cd browser &&  wasm-pack test --release --headless --firefox
	cargo test --all --all-features --no-fail-fast
	${MAKE} frontend-test

frontend-test:
	cd browser && wasm-pack test --release --headless --chrome
	cd browser &&  wasm-pack test --release --headless --firefox
	yarn test

a:
	echo a
b:
	${MAKE} a

xml-test-coverage: migrations
	cd browser && cargo tarpaulin -t 1200 --out Xml
	cargo tarpaulin -t 1200 --out Xml

coverage: migrations
	cd browser && cargo tarpaulin -t 1200 --out Html
	cargo tarpaulin -t 1200 --out Html

release: frontend
	cargo build --release

clean:
	cargo clean
	yarn clean

docker-build:
	docker build -t mcaptcha/mcaptcha:master -t mcaptcha/mcaptcha:latest .

docker-publish: docker-build
	docker push mcaptcha/mcaptcha:master 
	docker push mcaptcha/mcaptcha:latest

migrations:
	cargo run --bin tests-migrate

help:
	@echo  '  clean                   - drop builds and environments'
	@echo  '  coverage                - build test coverage in HTML format'
	@echo  '  dev-env                 - download dependencies'
	@echo  '  docker-build            - build docker image'
	@echo  '  docker-publish          - build and publish docker image'
	@echo  '  doc                     - build documentation'
	@echo  '  frontend                - build static assets in prod mode'
	@echo  '  frontend-test           - run frontend tests'
	@echo  '  migrations              - run database migrations'
	@echo  '  run                     - run developer instance'
	@echo  '  test                    - run unit and integration tests'
	@echo  '  xml-coverage            - build test coverage in XML for upload to codecov'
	@echo  ''
