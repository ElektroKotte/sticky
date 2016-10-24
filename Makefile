.PHONY: all debug release serve

MODE ?= release
release_flags = --release

all: release

debug release:
	make MODE=$@ build

build:
	cargo build $($(MODE)_flags)

target/debug/sticky: $(MODE)

serve: target/debug/sticky
	mkdir -p cgi-bin
	cp target/debug/sticky cgi-bin
	python -m CGIHTTPServer 7777
