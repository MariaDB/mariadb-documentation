import argparse
import logging
from dataclasses import dataclass
from pathlib import Path
from typing import Any, NamedTuple

import toml

DEFAULT_HTML_PATH = "output.html"
DEFAULT_PDF_PATH = "MariaDBServerKnowledgeBase.pdf"
DEFAULT_VERBOSITY = 1


class TocTypeConfig(NamedTuple):
    font_size: str
    padding_left: str
    margin: str


class TocConfig(NamedTuple):
    chapter: TocTypeConfig
    main: TocTypeConfig


# Public
class Config(NamedTuple):
    port: int
    pdf: bool
    verbose: bool
    repeat_outline: bool
    languages: list[str]
    num_rows: int
    pdf_path: Path
    html_path: Path
    wkhtml_settings: dict[str, Any]
    toc_config: TocConfig


def read_config() -> Config:
    """Returns a simplified data structure containing the config settings"""
    arg_config = _read_args()
    dict_config: dict[str, Any] = toml.load(arg_config.config)
    config: Config = generate_config(arg_config, dict_config)

    return config


@dataclass
class _ArgConfig:
    config: str
    port: int
    nopdf: bool
    norepeat: bool
    htmlpath: str
    pdfpath: str
    langs: list[str]
    numrows: int
    quiet: bool


def generate_config(arg_config: _ArgConfig, dict_config: dict[str, Any]) -> Config:
    return Config(
        port=arg_config.port,
        pdf=not arg_config.nopdf,
        verbose=not arg_config.quiet,
        repeat_outline=not arg_config.norepeat,
        languages=["en"] if not arg_config.langs else arg_config.langs,
        num_rows=-1 if arg_config.numrows is None else arg_config.numrows,
        html_path=Path(
            DEFAULT_HTML_PATH if arg_config.htmlpath is None else arg_config.htmlpath
        ),
        pdf_path=Path(
            DEFAULT_PDF_PATH if arg_config.pdfpath is None else arg_config.pdfpath
        ),
        toc_config=read_toc_config(dict_config["TOC"]),
        wkhtml_settings=dict_config["wkhtmltopdf"],
    )


def _read_args() -> _ArgConfig:
    """Parses and return the information from system arguments"""
    parser = argparse.ArgumentParser()

    # verbosity
    group = parser.add_mutually_exclusive_group()
    group.add_argument(
        "-q", "--quiet", action="store_true", help="Quiet/Hide Progress Bar"
    )

    parser.add_argument(
        "-c",
        "--config",
        type=Path,
        default=Path("./config.toml"),
        help="config filepath",
    )
    parser.add_argument(
        "-p", "--port", type=int, default=7032, help="mariadb_kb_server port"
    )
    parser.add_argument(
        "-l", "--langs", type=str, nargs="+", help="Optional Languages eg: (en, it)"
    )
    parser.add_argument(
        "-n",
        "--numrows",
        "--num_rows",
        type=int,
        help="Maximum Number of csv urls to use.",
    )
    parser.add_argument(
        "--norepeat", action="store_true", help="Turns off repeat generation"
    )
    parser.add_argument("--nopdf", action="store_true", help="Turns off pdf generation")
    parser.add_argument("-o", "--pdfpath", type=str, help="Path to write Final PDF")
    parser.add_argument(
        "--htmlpath", "--html_path", type=str, help="Path to write HTML Output"
    )
    parser.add_argument(
        "-d",
        "--debug",
        help="Print lots of debugging statements",
        action="store_const",
        dest="loglevel",
        const=logging.DEBUG,
        default=logging.WARNING,
    )
    parser.add_argument(
        "-v",
        "--verbose",
        help="Be verbose",
        action="store_const",
        dest="loglevel",
        const=logging.INFO,
    )
    args = parser.parse_args()
    logging.basicConfig(level=args.loglevel)
    return parser.parse_args(namespace=_ArgConfig)  # type: ignore


def read_toc_config(config: dict[str, Any]) -> TocConfig:
    main = TocTypeConfig(
        font_size=config["main_font_size"],
        padding_left=config["main_indent"],
        margin=config["main_margin"],
    )
    chapter = TocTypeConfig(
        font_size=config["chapter_font_size"],
        padding_left=config["chapter_indent"],
        margin=config["chapter_margin"],
    )
    return TocConfig(main=main, chapter=chapter)
