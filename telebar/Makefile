include config.mk

.DEFAULT_TARGET := build/release/main
build/release/main: ## Create the production binary
	@ cargo build --release --target-dir ${TELEBAR_RELEASE_BUILD_DIR}

.PHONY: install
install: build/release/main ## Install the production binary onto this system
	@ echo "WRITE ME"

.PHONY: help
help: ## Display this help message
	@ grep -E '^[/a-zA-Z_-]+:.*?## .*$$' Makefile | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: test
test: ## Run all tests
	@ cargo test

.PHONY: example
example: ## Start up an example telebar-server
	@ cargo run -- --output xsetroot --id ${TELEBAR_EXAMPLE_SOCKET_ID}
