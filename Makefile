BUNDLE = static/cache/bundle
OPENAPI = docs/openapi
CLEAN_UP = $(BUNDLE) src/cache_buster_data.json assets

define deploy_dependencies ## deploy dependencies
	docker create --name ${db}  \
		-e POSTGRES_PASSWORD=password  \
		-p 5432:5432   \
		postgres
	docker create  \
		-p 3306:3306  \
		--name ${mdb} \
		--env MARIADB_USER=maria \
		--env MARIADB_PASSWORD=password  \
		--env MARIADB_ROOT_PASSWORD=password  \
		--env MARIADB_DATABASE=maria  \
		mariadb:latest
	docker create  \
		-p 6379:6379 \
		--name mcaptcha-cache \
		mcaptcha/cache:latest
	docker start ${db}
	docker start ${mdb}
	docker start mcaptcha-cache
endef

define run_migrations ## run database migrations
	cd db/db-migrations/ && cargo run
endef

define frontend_env ## install frontend deps
	yarn install
	cd docs/openapi && yarn install
endef

define cache_bust ## run cache_busting program
	cd utils/cache-bust && cargo run
endef

default: frontend ## Build app in debug mode
	$(call cache_bust)
	cargo build

check: ## Check for syntax errors on all workspaces
	cargo check --workspace --tests --all-features
	cd utils/cache-bust && cargo check --tests --all-features
	cd db/db-migrations && cargo check --tests --all-features
	cd db/db-sqlx-postgres &&\
		DATABASE_URL=${POSTGRES_DATABASE_URL}\
		cargo check
	cd db/db-sqlx-maria &&\
		DATABASE_URL=${MARIA_DATABASE_URL}\
		cargo check
	cd db/db-core/ && cargo check

cache-bust: ## Run cache buster on static assets
	$(call cache_bust)

clean: ## Delete build artifacts
	@cargo clean
	@yarn cache clean
	@-rm $(CLEAN_UP)

coverage: migrate ## Generate code coverage report in HTML format
	$(call cache_bust)
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

env.db: ## Deploy dependencies
	$(call deploy_dependencies)
	sleep 5
	$(call run_migrations)

env.db.recreate: ## Deploy dependencies from scratch
	@-docker rm -f ${db}
	@-docker rm -f ${mdb}
	@-docker rm -f mcaptcha-cache
	$(call deploy_dependencies)
	sleep 5
	$(call run_migrations)

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
	$(call run_migrations)

release: frontend ## Build app with release optimizations
	$(call cache_bust)
	cargo build --release

run: frontend ## Run app in debug mode
	$(call cache_bust)
	cargo run


sqlx-offline-data: ## prepare sqlx offline data
	cd db/db-sqlx-postgres && cargo sqlx prepare  \
		--database-url=${POSTGRES_DATABASE_URL} -- \
		--all-features
	cd db/db-sqlx-maria && cargo sqlx prepare  \
		--database-url=${MARIA_DATABASE_URL} -- \
		--all-features
#	cd db/db-sqlx-sqlite/ \
#		&& DATABASE_URL=${SQLITE_DATABASE_URL} cargo sqlx prepare

test-db: ## run tests on database
	cd db/db-sqlx-postgres &&\
		DATABASE_URL=${POSTGRES_DATABASE_URL}\
		cargo test --no-fail-fast
test: frontend-test frontend ## Run all available tests
	$(call cache_bust)
	cd db/db-sqlx-postgres &&\
		DATABASE_URL=${POSTGRES_DATABASE_URL}\
		cargo test --no-fail-fast
	cd db/db-sqlx-maria &&\
		DATABASE_URL=${MARIA_DATABASE_URL}\
		cargo test --no-fail-fast
	cargo test --no-fail-fast
#	./scripts/tests.sh

test.integration: ## run integration tests with nightwatch.js
	./scripts/integration.sh

xml-test-coverage: migrate ## Generate code coverage report in XML format
	$(call cache_bust)
	cargo tarpaulin -t 1200 --out Xml

help: ## Prints help for targets with comments
	@cat $(MAKEFILE_LIST) | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
