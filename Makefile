DOCKER = docker run --rm --platform linux/amd64 -v "$(PWD)":/work -w /work rust:latest

test:
	$(DOCKER) cargo test

build:
	$(DOCKER) cargo build

shell:
	docker run --rm -it --platform linux/amd64 -v "$(PWD)":/work -w /work rust:latest bash

clean:
	rm -rf target

.PHONY: test build shell clean
