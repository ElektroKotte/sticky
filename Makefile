.PHONY: all debug release serve certs docker

MODE ?= release
release_flags = --release

all: release

debug release:
	make MODE=$@ build

build:
	cargo build $($(MODE)_flags)

certs:
	$(MAKE) -C certs

target/debug/sticky: $(MODE)

serve: target/debug/sticky

docker: certs release
	docker build -t sticky .

run: docker
	docker run -p 7010:7010 sticky
