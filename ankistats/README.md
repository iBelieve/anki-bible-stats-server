# Anki Bible Stats

A simple web server (and CLI tool) for getting Anki stats from an Anki database (such as one managed by the self-hosted Anki server). This project as a whole is specifically designed for my Bible memorization workflow, with specific deck names and card types hardcoded. So it probably wouldn't be useful for someone else to directly use. But I like open-sourcing things I work on to share and showcase what I build and in case any specific bits might be useful, such as as Anki database querying.

### Running Locally

You can run both the server locally, but there's also a CLI that prints the same data the API returns but in pretty tables. You just need to pass it the path to an Anki database. Alternatively, you can copy your `collection.anki2` to the repo root (it'll be gitignored), and run `make` commands for the different CLI commands:

```
make books
make today
make daily
```

### Testing

You can run the test suite by running `cargo test`. To test the verse and book name parsers against real Bible references, run:

```
./target/debug/cli refs /path/to/collection.anki2 > tests/data/bible_references.txt
```

Alternatively, if you've copied your `collection.anki2` to the repo root, you can run `make test` and it will automatically generate the test data file and run the test suite.

### Deploying

This project is built and deployed as a Docker image built from `Dockerfile`. It can be deployed to any container hosting platform or orchestrator. I am using [Dokku](https://dokku.com/). You will need to mount a volume with your Anki database, and set two env vars:

```
ANKI_DATABASE_PATH:  /path/to/collection.anki2
API_KEY:             ...
```

The `API_KEY` is used to secure API endpoints, and can be passed to HTTP requests as the `Authentication: Bearer <api-key>` header.

### Using With the Self-Hosted Anki Server

If you want to use this project in combination with the self-hosted Anki server, you will immediately run into an issue where the Anki database is locked. An error might look like:

```
Error: Failed to find deck 'BibleVerses': database is locked: Error code 5: The database file is locked
```

This is because Anki opens the database in exclusive locking mode:

```rust
fn open_or_create_collection_db(path: &Path) -> Result<Connection> {
    ...

    db.pragma_update(None, "locking_mode", "exclusive")?;

    ...
}
```

See https://github.com/ankitects/anki/blob/d24d2e33943af2361b5a9880572b30887efcf3ee/rslib/src/storage/sqlite.rs#L65

My solution has been to fork Anki and comment out this line. You can find my fork and the patch at https://github.com/ibelieve/anki/.

### License

This project is licensed under the [GNU Affero General Public License v3.0 or later](https://www.gnu.org/licenses/agpl-3.0.en.html).
