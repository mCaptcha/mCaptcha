default: frontend
	cargo build

clean:
	cargo clean
	rm -rf browser/pkg || true
	rm ./src/cache_buster_data.json || true
	rm -rf ./static/cache/bundle || true
	rm -rf ./assets || true

coverage: migrate
	cd browser && cargo tarpaulin -t 1200 --out Html
	cargo tarpaulin -t 1200 --out Html

dev-env:
	cargo fetch
	yarn install

doc:
	#yarn doc
	cargo doc --no-deps --workspace --all-features
	cd browser && cargo doc --no-deps --workspace --all-features

docker-build:
	docker build -t mcaptcha/mcaptcha:master -t mcaptcha/mcaptcha:latest .

docker-publish: docker-build
	docker push mcaptcha/mcaptcha:master 
	docker push mcaptcha/mcaptcha:latest

frontend:
	cd browser && wasm-pack build --release
	yarn install
	yarn build

frontend-test:
	cd browser && wasm-pack test --release --headless --chrome
	cd browser &&  wasm-pack test --release --headless --firefox
	cd browser && cargo test
	yarn test

migrate:
	cargo run --bin tests-migrate

release: frontend
	cargo build --release

run: frontend
	cargo run

test: frontend-test frontend
	echo 'static/' && tree static || true
	echo 'tree/' && tree assets || true
	cargo test --all-features --no-fail-fast

xml-test-coverage: migrate
	cd browser && cargo tarpaulin -t 1200 --out Xml
	cargo tarpaulin -t 1200 --out Xml

help:
	@echo  '  clean                   - drop builds and environments'
	@echo  '  coverage                - build test coverage in HTML format'
	@echo  '  dev-env                 - download dependencies'
	@echo  '  docker-build            - build docker image'
	@echo  '  docker-publish          - build and publish docker image'
	@echo  '  doc                     - build documentation'
	@echo  '  frontend                - build static assets in prod mode'
	@echo  '  frontend-test           - run frontend tests'
	@echo  '  migrate                 - run database migrations'
	@echo  '  run                     - run developer instance'
	@echo  '  test                    - run unit and integration tests'
	@echo  '  xml-coverage            - build test coverage in XML for upload to codecov'
	@echo  ''
