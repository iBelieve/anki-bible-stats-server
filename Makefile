all: build

build:
	cargo build -p ankistats

test: ankistats/tests/data/bible_references.txt
	cargo test -p ankistats

ankistats/tests/data/bible_references.txt: target/debug/ankistats ankistats/collection.anki2
	./target/debug/ankistats refs ankistats/collection.anki2 > ankistats/tests/data/bible_references.txt

target/debug/ankistats:
	cargo build -p ankistats

books:
	cargo run -p ankistats -- books ankistats/collection.anki2

today:
	cargo run -p ankistats -- today ankistats/collection.anki2

daily:
	cargo run -p ankistats -- daily ankistats/collection.anki2

weekly:
	cargo run -p ankistats -- weekly ankistats/collection.anki2

.PHONY: backend
backend:
	ANKI_DATABASE_PATH=ankistats/collection.anki2 API_KEY=test cargo run -p backend
