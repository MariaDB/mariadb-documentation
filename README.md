# mariadb_crawler

A Program that recursively crawls the mariadb kb and saves it to disk.

## How to Run
- [Install Rust](https://www.rust-lang.org/tools/install).

- Make sure [mariadb_archive_server](https://github.com/Icerath/mariadb_archive_server) is running or run with the --force flag.

- run `'cargo run --release -- '` with no flags for full scrape

- run with the `'recent'` flag to only update recently changed articles.

Keep it mind cargo might take a while for the first compile.

Run with the --help flag to print the usage