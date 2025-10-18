all: build

build:
	cargo build

test: tests/data/bible_references.txt
	cargo test

tests/data/bible_references.txt: target/debug/cli collection.anki2
	./target/debug/cli refs collection.anki2 > tests/data/bible_references.txt

target/debug/cli:
	cargo build --bin cli
