# mariadb_kb_crawler

A Program that recursively crawls the mariadb kb and saves it to disk.

## Installation
- [Install Rust](https://www.rust-lang.org/tools/install).

- Run `'git clone https://github.com/Icerath/mariadb_kb_crawler'`
- cd into `'mariadb_kb_crawler'`

- Build the binary with `'cargo build --release'`

- The output file will be `'target/release/mariadb_kb_crawler(.exe)'`

Keep in mind the first build might take some time.
## Usage
- Make sure [mariadb_kb_server](https://github.com/Icerath/mariadb_kb_server) is running or use the --force flag.

- Run `'./target/release/mariadb_kb_crawler(.exe)'` (make sure you're in mariadb_kb_crawler/)

- the `'--help'` flag prints the usage.

- the `'recent'` argument (no '`--`') will only request files updated from the last run.

- the `'--force'` flag prevents needing [mariadb_kb_server](https://github.com/Icerath/mariadb_kb_server).

## Example Installation and usage
Does not include the rust installation or server installation   


### Create new directory (optional)
```
mkdir mariadb_kb
cd mariadb_kb
```

### Install/Run the server (optional)
```
git clone https://github.com/Icerath/mariadb_kb_server
```
Then follow the rest of the instructions from [mariadb_kb_server](https://github.com/Icerath/mariadb_kb_server).

### Clone, Build and Run
```
git clone https://github.com/Icerath/mariadb_kb_crawler
cd mariadb_kb_crawler

cargo build --release

./target/release/mariadb_kb_crawler(.exe) --force --verbose

```

### Full Installation and Setup

mariadb_kb_crawler is not particularly useful on it's own, it's part of a group of repos which end up relying on this one.

I use 'KB' to refer to the [MariaDB Knowledgebase](https://mariadb.com/kb/)

- [mariadb_kb_archive](https://github.com/Icerath/mariadb_kb_archive) contains content from the KB as well as a file containing when it was last updated.
- [mariadb_kb_server](https://github.com/Icerath/mariadb_kb_server) is a binary that directs knowledge base urls to their static html, it also provides the kb_urls.csv which keeps track of a bunch of different articles from the KB to use with [mariadb_pdf](https://github.com/Icerath/https://github.com/Icerath/mariadb_pdf).
<br>
The [mariadb_kb_server](https://github.com/Icerath/mariadb_kb_server) is only used by the crawler to read kb_urls.csv, as it contains a few KB redirects which are not located KB directly.
<br>
If you wish to use the crawler without running the [mariadb_kb_server](https://github.com/Icerath/mariadb_kb_server), you can do so by passing the `--force` flag.
- [mariadb_pdf](https://github.com/Icerath/mariadb_pdf) is a python script which uses [wkhtmltopdf](https://wkhtmltopdf.org/) to turn the KB into a single pdf file for offline use.
- [mariadb_help_contents](https://github.com/Icerath/mariadb_help_contents) // TODO