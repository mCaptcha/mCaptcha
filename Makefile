BUNDLE = static/cache/bundle
OPENAPI = docs/openapi
CLEAN_UP = $(BUNDLE) src/cache_buster_data.json assets

define frontend_env ## install frontend deps
	yarn install
	cd docs/openapi && yarn install
endef

default: frontend ## Build app in debug mode
	cargo build

check: ## Check for syntax errors on all workspaces
	cargo check --workspace --tests --all-features
	cd db/db-migrations && cargo check --tests --all-features
	cd db/db-sqlx-postgres &&\
		DATABASE_URL=${POSTGRES_DATABASE_URL}\
		cargo check
	cd db/db-core/ && cargo check

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
	$(call frontend_env)

frontend-env: ## Install frontend deps
	$(call frontend_env)

frontend: ## Build frontend
	$(call frontend_env)
	cd $(OPENAPI) && yarn build
	yarn install
	@-rm -rf $(BUNDLE)
	@-mkdir $(BUNDLE)
	yarn build
	@yarn run sass -s \
		compressed templates/main.scss  \
		./static/cache/bundle/css/main.css
	@yarn run sass -s \
		compressed templates/mobile.scss  \
		./static/cache/bundle/css/mobile.css
	@yarn run sass -s \
		compressed templates/widget/main.scss  \
		./static/cache/bundle/css/widget.css
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
	cd db/db-migrations/ && cargo run

release: frontend ## Build app with release optimizations
	cargo build --release

run: frontend ## Run app in debug mode
	cargo run


sqlx-offline-data: ## prepare sqlx offline data
	cargo sqlx prepare  --database-url=${POSTGRES_DATABASE_URL} -- --bin mcaptcha \
		--all-features
	cd db/db-migrations && cargo sqlx prepare  \
		--database-url=${POSTGRES_DATABASE_URL} -- --bin db-migrations \
		--all-features
#	cd db/db-sqlx-sqlite/ \
#		&& DATABASE_URL=${SQLITE_DATABASE_URL} cargo sqlx prepare

test: frontend-test frontend ## Run all available tests
	cd db/db-sqlx-postgres &&\
		DATABASE_URL=${POSTGRES_DATABASE_URL}\
		cargo test --no-fail-fast
#	./scripts/tests.sh
#	cargo test --all-features --no-fail-fast

xml-test-coverage: migrate ## Generate code coverage report in XML format
	cargo tarpaulin -t 1200 --out Xml

help: ## Prints help for targets with comments
	@cat $(MAKEFILE_LIST) | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
