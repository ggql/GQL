# Config

VERSION=$(version)


# Build

.PHONY: FORCE

build: rs-build
.PHONY: build

clean: rs-clean
.PHONY: clean

install: rs-install
.PHONY: install

lint: rs-lint
.PHONY: lint

test: rs-test
.PHONY: test


# Non-PHONY targets (real files)

rs-build: FORCE
	./scripts/build.sh $(VERSION)

rs-clean: FORCE
	./scripts/clean.sh

rs-install: FORCE
	./scripts/install.sh

rs-lint: FORCE
	./scripts/lint.sh

rs-test: FORCE
	./scripts/test.sh
