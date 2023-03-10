from setup.kb_urls import CsvItem, apply_depth
from setup.config import Config
from setup.paths import DIR_PATH_STR
from setup.paths import get_html

from copy import copy
from bs4 import BeautifulSoup, Tag

def read_languages(en_csv: list[CsvItem], config: Config) -> dict[str, list[CsvItem]]:
    language_csvs = {
        lang: [] for lang in config.languages 
    }

    if "en" in config.languages:
        language_csvs["en"] = en_csv
    
    if config.languages == ["en"]:
        return language_csvs

    for row in en_csv:
        found_languages = _find_languages(row)
        for lang, lang_val in found_languages.items():
            if lang in config.languages:
                new_row = _create_lang_row(row, lang_val)
                if new_row not in language_csvs[lang]:
                    language_csvs[lang].append(new_row)
    for csv in language_csvs.values():
        apply_depth(csv)
    return language_csvs


def _find_languages(row: CsvItem) -> dict[str, str]:
    html = get_html(row.url)
    if html == None:
        return {}
    soup = BeautifulSoup(html, features="html.parser")
    header = soup.find(["h3","h4","h5","h6"], text="Localized Versions")
    if header is None:
        return {}
    assert isinstance(header, Tag)
    versions_div = header.parent.find_next_sibling() # type: ignore
    if versions_div is None:
        return {}
    assert isinstance(versions_div, Tag)
    languages = {}
    for li in versions_div.select("li"):
        if isinstance(li.a, Tag) and "href" in li.a.attrs:
            lang_url = li.a.attrs["href"]
            li.a.decompose()
            lang = li.text.strip().removeprefix("[").removesuffix("]")
            languages[lang] = lang_url
            assert lang_url.startswith(f"/kb/{lang}")
    return languages

def _create_lang_row(row: CsvItem, url: str) -> CsvItem:
    new_row = copy(row)
    new_row.url = url
    row.id = url
    return new_row