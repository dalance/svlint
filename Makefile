
VERSION ?= v$(patsubst "%",%, $(word 3, $(shell grep version Cargo.toml)))
REPOSITORY := $(patsubst "%",%, $(word 3, $(shell grep repository Cargo.toml)))
GIT_REVISION := $(shell git rev-parse --short=8 HEAD)
DATE_ISO8601 := $(shell date +"%Y-%m-%d")
TIME_ISO8601 := $(shell date +"%H:%M:%S")
DATETIME_ISO8601 := ${DATE_ISO8601}T${TIME_ISO8601}
RUST_VERSION := $(word 2, $(shell rustc -V))
LONG_VERSION := ${VERSION} rev:${GIT_REVISION} rustc:${RUST_VERSION} built:${DATETIME_ISO8601}
BIN_NAME := svlint

export LONG_VERSION

.PHONY: all test clean release_lnx release_win release_mac

all: test

test:
	cargo test

watch:
	cargo watch test

clean:
	cargo clean

# Flags used by both development and release versions of PDF.
# NOTE: If you change these, be sure to reflect the changes in the release and
# mdgen workflow files too.
PANDOC_FLAGS := -f markdown+lists_without_preceding_blankline
PANDOC_FLAGS += --template=md/MANUAL_template.tex
PANDOC_FLAGS += --metadata "title=Svlint Manual"
PANDOC_FLAGS += --metadata "author=${REPOSITORY}"
PANDOC_FLAGS += --metadata "keywords=SystemVerilog"
PANDOC_FLAGS += --metadata "keywords=Verilog"
PANDOC_FLAGS += --metadata "keywords=IEEE1800-2017"
PANDOC_FLAGS += --metadata "keywords=lint"
PANDOC_FLAGS += --metadata "keywords=svlint"
PANDOC_FLAGS += --metadata "keywords=sv-parser"
PANDOC_FLAGS += --metadata "keywords=svls"
PANDOC_FLAGS += --toc
PANDOC_FLAGS += --toc-depth=2
PANDOC_FLAGS += --variable=colorlinks
PANDOC_FLAGS += --variable=papersize:a4

# Minor tweaks for nicer formatting of PDF.
#   - Begin each rule,ruleset description on a new page.
#   - Compact form for rule's hint and reason.
MANUAL.intermediateTex.md:
	sed \
		-e 's/^## Syntax Rule: /\\clearpage\n## Syntax Rule: /' \
		-e 's/^## Ruleset: /\\clearpage\n## Ruleset: /' \
		-e '/^### Hint$$/{$$!{N;N;s/### Hint\n\n/Hint\n: /;t;P;D}}' \
		-e '/^### Reason$$/{$$!{N;N;s/### Reason\n\n/Reason\n: /;t;P;D}}' \
		MANUAL.md > $@

# Convenience recipe for building development version of PDF manual.
# This is normally handled by the GitHub Action `.github/workflows/mdgen.yml`
# which runs on pushes and pull requests, and does NOT use this recipe.
.PHONY: MANUAL-dev
MANUAL-dev: MANUAL.intermediateTex.md
	@pandoc --version
	pandoc -i MANUAL.intermediateTex.md \
		${PANDOC_FLAGS} \
		--metadata "subtitle=DEVELOPMENT ${GIT_REVISION}" \
		--metadata "date=${DATETIME_ISO8601}" \
		-o MANUAL-dev.pdf
	rm -f *.intermediate*.*

# Convenience recipe for building release version of PDF manual.
# This is normally handled by the GitHub Action `.github/workflows/release.yml`
# which runs when a new tag `v*.*.*` is pushed, and does NOT use this recipe.
.PHONY: MANUAL-release
MANUAL-release: MANUAL.intermediateTex.md
	pandoc -i MANUAL.intermediateTex.md \
		${PANDOC_FLAGS} \
		--metadata "subtitle=${VERSION}" \
		--metadata "date=${DATE_ISO8601}" \
		-o MANUAL-release.pdf
	rm -f *.intermediate*.*

# The `release` action should create a file of this name and upload it as an
# artifact in a prerequisite job before the parallel jobs (Linux, Windows,
# MacOS) download the artifact, build executables, and create GitHub releases.
RELEASE_MANUAL := pdf/svlint_MANUAL_${VERSION}.pdf

release_lnx:
	#cargo build --release --target=x86_64-unknown-linux-musl
	mkdir -p target/x86_64-unknown-linux-musl/release/
	echo FOO > target/x86_64-unknown-linux-musl/release/${BIN_NAME}
	# TODO: Undo above
	rm -rf tmp
	mkdir -p tmp/bin/ tmp/doc/
	cp ${RELEASE_MANUAL} tmp/doc/
	cp rulesets/*.toml tmp/bin/
	cp $$(find rulesets/ -type f -perm -u+x) tmp/bin/
	cp target/x86_64-unknown-linux-musl/release/${BIN_NAME} tmp/bin/
	cd tmp/ && \
		zip ${BIN_NAME}-${VERSION}-x86_64-lnx.zip -r *
	mv tmp/${BIN_NAME}-${VERSION}-x86_64-lnx.zip ./
	rm -rf tmp/

release_win:
	#cargo build --release --target=x86_64-pc-windows-msvc
	mkdir -p target/x86_64-pc-windows-msvc/release/
	echo FOO > target/x86_64-pc-windows-msvc/release/${BIN_NAME}.exe
	# TODO: Undo above
	rm -rf tmp
	mkdir -p tmp/bin/ tmp/doc/
	cp ${RELEASE_MANUAL} tmp/doc/
	cp rulesets/*.toml tmp/bin/
	cp rulesets/*.cmd tmp/bin/
	cp target/x86_64-pc-windows-msvc/release/${BIN_NAME}.exe tmp/bin/
	cd tmp && \
		7z a ${BIN_NAME}-${VERSION}-x86_64-win.zip *
	mv tmp/${BIN_NAME}-${VERSION}-x86_64-win.zip ./
	rm -rf tmp/

release_mac:
	#cargo build --release --target=x86_64-apple-darwin
	mkdir -p target/x86_64-apple-darwin/release/
	echo FOO > target/x86_64-apple-darwin/release/${BIN_NAME}
	# TODO: Undo above
	rm -rf tmp
	mkdir -p tmp/bin/ tmp/doc/
	cp ${RELEASE_MANUAL} tmp/doc/
	cp rulesets/*.toml tmp/bin/
	cp $$(find rulesets/ -type f -perm -u+x) tmp/bin/
	cp target/x86_64-apple-darwin/release/${BIN_NAME} tmp/bin/
	cd tmp/ && \
		zip ${BIN_NAME}-${VERSION}-x86_64-mac.zip -r *
	mv tmp/${BIN_NAME}-${VERSION}-x86_64-mac.zip ./
	rm -rf tmp/
