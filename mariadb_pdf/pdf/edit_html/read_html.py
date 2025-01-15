from setup.config import Config
from setup.kb_urls import CsvItem
from setup.logger import log
from setup.paths import get_html

from .contents import TocItem
from .merge_html import merge_html
from .process_html_page import process_html_page


def read_html(kburls: list[CsvItem], outline: list[TocItem], config: Config) -> str:
    pages = process_pages(kburls, outline, config)
    html = merge_html(pages, kburls, outline, config)
    return html


def process_pages(
    kburls: list[CsvItem], outline: list[TocItem], config: Config
) -> list[str]:
    html_pages: list[str] = []

    url_to_depth_str = {}
    for row in kburls:
        if row.include in [1, 2]:
            url_to_depth_str[row.url] = row.depth_str

    length = len(kburls)
    for index, (row, outline_row) in enumerate(zip(kburls, outline, strict=True)):
        logging.info("Start HTML read")
        if config.verbose:
            print(f"\rProgress: {index+1}/{length}", end="")
        assert row.include != 0

        if row.include == 2:
            html_tag = f'<h2 class="col-md-8;">{row.depth_str} {row.header}</h2>'
            outline_row.header = f"{row.depth_str} {row.header}"
        elif row.include == 3:
            depth_str = url_to_depth_str.get(row.url, row.depth_str)
            html_tag = f'<h2 class="col-md-8"><a href="{row.url}">{depth_str} {row.header}</a></h2>'
            outline_row.header = f"{row.depth_str} {row.header}"
        else:
            html_tag, header = process_article(row, config.port)
            outline_row.header = header
        html_pages.append(html_tag)
    if config.verbose:
        print(f"\rProgress: {length}/{length}")
    return html_pages


def process_article(row: CsvItem, port: int) -> tuple[str, str]:
    html = get_html(row.url, port)
    if html is None:
        log.error(f"Could not read url: {row.url}")
        exit(1)
    return process_html_page(html, row)

