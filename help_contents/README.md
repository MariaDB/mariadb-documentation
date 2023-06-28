# Mariadb-Help_Contents

## Creating Help Contents

Make sure [mariadb_kb_server](https://github.com/icerath/mariadb_kb_server) is running.

Make sure to install the dependencies, on Linux you can use `virtualenv` and the
following commands:

```console
virtualenv .venv
source .venv/bin/activate
pip3 install -r requirements.txt
```

Run `(python/python3) main.py -v [versions...]` to generate help contents.

You can specify multiple version numbers, eg:

```console
python main.py -v 1010 1011
```

Will generate help contents for versions 10.10, 10.11 and write them to:

```console
./output/fill_help_contents-1010.sql
./output/fill_help_contents-1011.sql
```

## Importing Help Contents

You can import the help contents file by running:

```console
mysql -u root -p mysql < [filepath]
```

where filepath is the path to the generated sql file.

## Dependencies

Python 3.10 +

### Libraries

See `requirements.txt`:

- bs4
- lxml
- requests
