include config.mk

.DEFAULT_TARGET = help
.PHONY: help
help: ## Display this help message
	@ grep -E '^[a-zA-Z_-]+:.*?## .*$$' Makefile | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: test
test: ## Run all unit, integration, and functional tests
	@ cargo test
	@ ./functional_tests/main.sh

.PHONY: example
example: ## Start up an example telebar-server
	@ cargo run -- --output xsetroot --id ${TELEBAR_EXAMPLE_SOCKET_ID}
