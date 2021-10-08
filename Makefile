default: frontend ## Run app in debug mode
	cargo run

clean: ## Delete build artifacts
	@cargo clean
	@yarn cache clean
	@-rm -rf browser/pkg
	@-rm ./src/cache_buster_data.json
	@-rm -rf ./static/cache/bundle
	@-rm -rf ./assets

coverage: migrate ## Generate code coverage report in HTML format
	cd browser && cargo tarpaulin -t 1200 --out Html
	cargo tarpaulin -t 1200 --out Html

dev-env: ## Setup development environtment
	cargo fetch
	yarn install
	cd docs/openapi && yarn install

doc: ## Generate documentation
	#yarn doc
	cargo doc --no-deps --workspace --all-features
	cd browser && cargo doc --no-deps --workspace --all-features

docker: ## Build Docker image
	docker build -t mcaptcha/mcaptcha:master -t mcaptcha/mcaptcha:latest .

docker-publish: docker ## Build and publish Docker image
	docker push mcaptcha/mcaptcha:master 
	docker push mcaptcha/mcaptcha:latest

frontend: ## Build frontend
	cd browser && wasm-pack build --release
	cd docs/openapi/ yarn build
	yarn install
	yarn build

frontend-test: ## Run frontend tests
	cd browser && wasm-pack test --release --headless --chrome
	cd browser &&  wasm-pack test --release --headless --firefox
	cd docs/openapi && yarn test
	cd browser && cargo test
	yarn test

lint: ## Lint codebase
	cargo fmt -v --all -- --emit files
	cargo clippy --workspace --tests --all-features
	yarn lint
	cd docs/openapi && yarn test

migrate: ## Run database migrations
	cargo run --bin tests-migrate

release: frontend ## Build app with release optimizations
	cargo build --release

test: frontend-test frontend ## Run all available tests
	echo 'static/' && tree static || true
	echo 'tree/' && tree assets || true
	cargo test --all-features --no-fail-fast

xml-test-coverage: migrate ## Generate code coverage report in XML format
	cd browser && cargo tarpaulin -t 1200 --out Xml
	cargo tarpaulin -t 1200 --out Xml

help: ## Prints help for targets with comments
	@cat $(MAKEFILE_LIST) | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
