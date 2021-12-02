BUNDLE = static/cache/bundle
OPENAPI = docs/openapi
CLEAN_UP = $(BUNDLE) src/cache_buster_data.json assets

default: frontend ## Build app in debug mode
	cargo build

clean: ## Delete build artifacts
	@cargo clean
	@yarn cache clean
	@-rm $(CLEAN_UP)

coverage: migrate ## Generate code coverage report in HTML format
	cargo tarpaulin -t 1200 --out Html

doc: ## Generate documentation
	#yarn doc
	cargo doc --no-deps --workspace --all-features

docker: ## Build Docker image
	docker build -t mcaptcha/mcaptcha:master -t mcaptcha/mcaptcha:latest .

docker-publish: docker ## Build and publish Docker image
	docker push mcaptcha/mcaptcha:master 
	docker push mcaptcha/mcaptcha:latest

env: ## Setup development environtment
	cargo fetch
	yarn install
	cd docs/openapi && yarn install

frontend: env ## Build frontend
	cd $(OPENAPI) && yarn build
	yarn install
	@-rm -rf $(BUNDLE)
	@-mkdir $(BUNDLE)
	yarn build
	@./scripts/librejs.sh
	@./scripts/cachebust.sh

frontend-test: ## Run frontend tests
	cd $(OPENAPI)&& yarn test
	yarn test

lint: ## Lint codebase
	cargo fmt -v --all -- --emit files
	cargo clippy --workspace --tests --all-features
	yarn lint
	cd $(OPENAPI)&& yarn test

migrate: ## Run database migrations
	cargo run --bin tests-migrate

release: frontend ## Build app with release optimizations
	cargo build --release

run: frontend ## Run app in debug mode
	cargo run

test: frontend-test frontend ## Run all available tests
	cargo test --all-features --no-fail-fast

xml-test-coverage: migrate ## Generate code coverage report in XML format
	cargo tarpaulin -t 1200 --out Xml

help: ## Prints help for targets with comments
	@cat $(MAKEFILE_LIST) | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
