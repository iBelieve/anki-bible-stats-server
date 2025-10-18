all: build

build:
	cargo build

test: tests/data/bible_references.txt
	cargo test

tests/data/bible_references.txt: target/debug/cli collection.anki2
	./target/debug/cli refs collection.anki2 > tests/data/bible_references.txt

target/debug/cli:
	cargo build --bin cli

books:
	cargo run --bin cli books collection.anki2

today:
	cargo run --bin cli today collection.anki2

daily:
	cargo run --bin cli daily collection.anki2

server:
	ANKI_DATABASE_PATH=collection.anki2 API_KEY=test cargo run --bin anki-bible-stats
