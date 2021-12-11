default: ## Builds the application in debug mode
	cargo build

clean: ## Delete build artifacts
	@cargo clean

doc: ## Generate documentation
	cargo doc --no-deps --workspace --all-features

docker: ## Builds the docker image for the bot
	docker build -t datadi/murgi_bot:latest .

docker-publish: docker ## Build and publish docker image
	docker push datadi/murgi_bot:latest

release: ## Build apps with release optimizations
	cargo build --release

lint: ## Lint codebase
	cargo fmt -v --all -- --emit files
	cargo clippy --workspace --tests --all-features

help: ## Prints help for targets with comments
	@cat $(MAKEFILE_LIST) | grep -E '^[a-zA-Z_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
