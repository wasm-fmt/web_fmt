set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set shell := ["bash", "-cu"]

_default:
	@just --list -u

alias test := test-rust

# Test commands
[group('test')]
test-rust:
	cargo test

[group('test')]
test-bun crate:
	bun test crates/{{crate}}/test_bun

[group('test')]
test-deno crate:
	deno test crates/{{crate}}/test_deno --allow-read --parallel

[group('test')]
test-node crate:
	node --test "crates/{{crate}}/test_node/*.mjs"

[group('test')]
test-wasm crate: (test-node crate) (test-deno crate) (test-bun crate)

[group('test')]
test-all: test-rust
	just test-wasm biome_fmt
	just test-wasm graphql_fmt
	just test-wasm json_fmt
	just test-wasm malva_fmt
	just test-wasm markup_fmt
	just test-wasm oxc_fmt
	just test-wasm web_fmt

# Build commands
[group('build')]
build crate:
	wasm-pack build --scope=wasm-fmt crates/{{crate}}
	cp -R crates/{{crate}}/extra/. crates/{{crate}}/pkg/
	node scripts/patch.mjs crates/{{crate}}

[group('build')]
build-all:
	just build biome_fmt
	just build graphql_fmt
	just build json_fmt
	just build malva_fmt
	just build markup_fmt
	just build oxc_fmt
	just build web_fmt

# Format commands
[group('fmt')]
fmt:
	cargo fmt --all
	taplo fmt .
	dprint fmt

# Check commands
[group('check')]
check:
	cargo check --all
	cargo clippy --all -- -D warnings
	cargo fmt --all --check
	taplo fmt --check .
	dprint check

# Audit command
[group('check')]
audit:
	cargo audit

# Ready command - check, build and test all
[group('release')]
ready:
	just check
	just build-all
	just test-all
	cd crates/biome_fmt/pkg && npm pack --dry-run
	cd crates/graphql_fmt/pkg && npm pack --dry-run
	cd crates/json_fmt/pkg && npm pack --dry-run
	cd crates/malva_fmt/pkg && npm pack --dry-run
	cd crates/markup_fmt/pkg && npm pack --dry-run
	cd crates/oxc_fmt/pkg && npm pack --dry-run
	cd crates/web_fmt/pkg && npm pack --dry-run

# Bump version (major, minor, patch) or set specific version
[group('release')]
version bump_or_version:
	#!/usr/bin/env bash
	if [[ "{{bump_or_version}}" =~ ^(major|minor|patch)$ ]]; then
		cargo set-version --bump "{{bump_or_version}}"
	else
		cargo set-version "{{bump_or_version}}"
	fi
	
	VERSION=$(cd crates/web_fmt && cargo pkgid | sed 's/.*[#@]//')
	node scripts/sync_version.mjs "${VERSION}"
	
	git add -A
	git commit -m "${VERSION}"
	git tag -a "v${VERSION}" -m "${VERSION}"
