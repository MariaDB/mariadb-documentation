# mariadb_kb_server

A small server for accessing the archive directly through urls and other small utilities.

## Installation
- [Install Rust](https://www.rust-lang.org/tools/install).

- Build the binary with `'cargo build --release'`

- The output file will be `'target/release/mariadb_kb_server(.exe)'`

- To access the KB files, either clone [mariadb_kb_archive](https://github.com/icerath/mariadb_kb_archive) into the same directory as this repo, or run the [mariadb_crawler](https://github.com/icerath/mariadb_kb_crawler).

Keep in mind the first build might take some time.

## Usage

- Run './target/release/mariadb_kb_server(.exe)'

- Run with the `'--help'` flag to print usage.

## Using the Server
- `localhost:[PORT]/kb/[url]` will respond with the original html and can be viewed like a regular webpage.
- `localhost:[PORT]/kb/[url]?list` will respond with all the subpages.
- `localhost:[PORT]/kb_urls.csv` will respond with the file's direct contents.
- `localhost:[PORT]/diff` will respond with the files missing from `kb_urls.csv`