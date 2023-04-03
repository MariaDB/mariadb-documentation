# mariadb_kb_crawler

A Program that recursively crawls the mariadb kb and saves it to disk.

## Installation
- [Install Rust](https://www.rust-lang.org/tools/install).

- Run `'git clone https://github.com/Icerath/mariadb_kb_crawler'`
- cd into `'mariadb_kb_crawler'`

- Build the binary with `'cargo build --release'`

- The output file will be `'target/release/mariadb_kb_crawler(.exe)'`

## Usage
- Make sure [mariadb_kb_server](https://github.com/Icerath/mariadb_mariadb_kb_server_server) is running or use the --force flag.

- Run './target/release/mariadb_kb_crawler(.exe)'

- the `'--help'` flag prints the usage.

- the `'recent'` argument (no '`--`') will only request files updated from the last run.

- the `'--force'` flag prevents needing mariadb_kb_server.

Keep it mind the first build might take some time.

Run with the --help flag to print the usage

## Example Installation and usage
Does not include the rust installation or server installation   


### Create new directory (optional)
```
mkdir mariadb_kb
cd mariadb_kb
```

### Clone, Build and Run
```
git clone https://github.com/Icerath/mariadb_kb_crawler
cd mariadb_kb_crawler

cargo build --release

./target/release/mariadb_kb_crawler(.exe) --force --verbose

```