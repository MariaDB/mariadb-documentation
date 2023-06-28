# Mariadb-Help_Contents

## Creating Help Contents
Make sure 
[mariadb_archive_server](https://github.com/icerath/mariadb_archive_server)
is running.

Make sure to install the dependencies

Run `(python/python3) main.py -v [versions...]` to generate help contents.

you can specify multiple version numbers, eg:

`python main.py -v 1010 1011`

will generate help contents for versions 10.10, 10.11 and write them to:

`output/fill_help_contents-1010.sql`

`output/fill_help_contents-1011.sql`

## Importing Help Contents

You can import the help contents file by running:

```mysql -u root -p mysql < [filepath]```

where filepath is the path to the generated sql file.

## Dependencies
Python 3.10 +

### Libraries
bs4

lxml

requests