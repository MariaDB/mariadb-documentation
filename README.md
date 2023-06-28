# mariadb_pdf

## Installation

- install [python](https://www.python.org/downloads/) `(3.10+)`
- install [wkhtmltopdf](https://wkhtmltopdf.org/downloads.html) (if on windows copy wkhtmltopdf.exe to this directory)
- Install python packages with (pip/pip3):
    - toml
    - pdfkit
    - requests
    - bs4

If you are on Linux (not tested on MacOS or Windows), you can setup a virtual
environment with the following commands (needs `virtualenv`):

```console
virtualenv .venv
source .venv/bin/activate
pip3 install -r requirements.txt
```

##  Usage

- make sure [mariadb_kb_server](https://github.com/icerath/mariadb_kb_server)
is running.
- run `'(python/python3) main.py'`

## Dependencies

See [requirements.txt](./requirements.txt).

### Libraries

- toml
- pdfkit
- requests
- bs4
