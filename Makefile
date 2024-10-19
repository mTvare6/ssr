all: build

build:
	MALLOC_CONF="thp:always,metadata_thp:always" cargo build --release

run:
	./target/release/ssr

install:
	cargo install --path .
