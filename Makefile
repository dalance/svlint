VERSION = $(patsubst "%",%, $(word 3, $(shell grep version Cargo.toml)))
REPOSITORY = $(patsubst "%",%, $(word 3, $(shell grep repository Cargo.toml)))
GIT_REVISION = $(shell git rev-parse --short=8 HEAD)
DATE = $(shell date +"%Y-%m-%d")
TIME = $(shell date +"%H:%M:%S")
DATETIME_ISO8601 = ${DATE}T${TIME}
RUST_VERSION = $(word 2, $(shell rustc -V))
LONG_VERSION = "${VERSION} ( rev: ${GIT_REVISION}, rustc: ${RUST_VERSION}, built: ${DATETIME_ISO8601} )"
BIN_NAME = svlint

export LONG_VERSION

.PHONY: all test clean release_lnx release_win release_mac

all: test

test:
	cargo test

watch:
	cargo watch test

clean:
	cargo clean

# Convenience recipe for building non-release version of PDF manual.
# This is normally handled by the GitHub Action `.github/workflows/mdgen.yml`
# which runs on pushes and pull requests, and does NOT use this recipe.
# TODO: Translate recipe to `mdgen.yml`.
.PHONY: MANUAL.pdf
MANUAL.pdf:
	@pandoc --version
	pandoc -i MANUAL.md \
		--template=md/MANUAL_template.tex \
		--metadata "title=Svlint Manual" \
		--metadata "subtitle=DEVELOPMENT ${GIT_REVISION}" \
		--metadata "author=${REPOSITORY}" \
		--metadata "date=${DATETIME_ISO8601}" \
		--metadata "keywords=SystemVerilog" \
		--metadata "keywords=Verilog" \
		--metadata "keywords=IEEE1800-2017" \
		--metadata "keywords=lint" \
		--metadata "keywords=svlint" \
		--metadata "keywords=sv-parser" \
		--metadata "keywords=svls" \
		--toc \
		-o MANUAL.pdf

# Convenience recipe for building release version of PDF manual.
# This is normally handled by the GitHub Action `.github/workflows/release.yml`
# which runs when a new tag `v*.*.*` is pushed, and does NOT use this recipe.
# TODO: Title page with latest tag and date.
# TODO: Translate recipe to `release.yml`.
.PHONY: MANUAL_release.pdf
MANUAL_release.pdf:
	pandoc -i MANUAL_release.md -o MANUAL_release.pdf

# The `release` action should create a file of this name and upload it as an
# artifact in a prerequisite job before the parallel jobs (Linux, Windows,
# MacOS) download the artifact, build executables, and create GitHub releases.
# RELEASE_MANUAL is created instead of passing the glob directly to `release_*`
# recipies in order to gracefully handle the cases where no files match the
# glob (ignore) or multiple files match (take the alphabetically last).
RELEASE_MANUAL := $(lastword $(wildcard svlint_MANUAL_v*.*.*.pdf))

release_lnx:
	cargo build --release --target=x86_64-unknown-linux-musl
	zip -j ${BIN_NAME}-v${VERSION}-x86_64-lnx.zip \
		${RELEASE_MANUAL} \
		target/x86_64-unknown-linux-musl/release/${BIN_NAME}

release_win:
	cargo build --release --target=x86_64-pc-windows-msvc
	7z a ${BIN_NAME}-v${VERSION}-x86_64-win.zip \
		${RELEASE_MANUAL} \
		target/x86_64-pc-windows-msvc/release/${BIN_NAME}.exe

release_mac:
	cargo build --release --target=x86_64-apple-darwin
	zip -j ${BIN_NAME}-v${VERSION}-x86_64-mac.zip \
		${RELEASE_MANUAL} \
		target/x86_64-apple-darwin/release/${BIN_NAME}
