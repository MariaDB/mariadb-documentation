from setup.logger import log
from setup.kb_urls import CsvItem

from bs4 import BeautifulSoup as Soup
from bs4 import Tag

PAGE_BREAK = '<div style = "page-break-after:always;"></div>\n'


def process_html_page(html: str, row: CsvItem) -> tuple[str, str]:
    section = extract_section(html, row.url)
    if not section: return "No Section", ""
    soup = Soup(section, features="html.parser")
    header = update_h1(soup, row)
    remove_section_id(soup)
    remove_unwanted(soup)
    remove_button_overlay(soup)
    flatten_subcontents(soup)
    remove_border_radius(soup)
    raise_from_mariadb(soup)
    remove_numbered_headers(soup, row)
    html = str(soup)
    html = externalise_ids(html, row)
    return (html, header)

def update_h1(soup: Soup, row: CsvItem) -> str:
    h1 = soup.select_one("h1")
    if h1 is not None:
        h1.attrs["id"] = "" # will be replaced by 'externalise_ids'
        assert isinstance(h1.string, str)
        h1.string = row.depth_str + ' ' + h1.string.strip()
        return h1.string #type: ignore
    return "Could not find h1"

def remove_section_id(soup: Soup):
    tag = soup.select_one("section")
    assert tag is not None
    tag.attrs.pop("id", None)

def extract_section(html: str, url: str) -> str:
    start_idx = html.find("<section")
    end_idx = html.find("</section>")
    if -1 in [start_idx, end_idx]:
        log.warning(f"Section not found for url: {url}")
        return ""
    return html[start_idx:end_idx]



def remove_unwanted(soup: Soup):
    selectors = [
        "div#content_disclaimer",
        "div#comments",
        "div#subscribe",
        "div.simple_section_nav",
        "div.node-breadcrumb",
    ]

    for item in selectors:
        for tag in soup.select(item):
            tag.decompose()

    for tag in soup.find_all("h2", text="Comments"):
        assert isinstance(tag, Tag)
        tag.decompose()
    

    for tag in soup.select("h2#see-also,h3#see-also"):
        for sibling in tag.find_next_siblings():
            assert isinstance(sibling, Tag)
            sibling.decompose()
        tag.decompose()


    # Remove links to see-also
    for tag in soup.find_all("a", attrs={"href": "#see-also"}):
        assert isinstance(tag, Tag)
        if isinstance(tag.parent, Tag) and tag.parent.name == "li":
            tag.parent.decompose()

def remove_button_overlay(soup: Soup):
    for button in soup.select(".btn.disabled"):
        assert isinstance(button["class"], list) # should contain "btn" and "disabled"
        button["class"].remove("disabled") # type: ignore 

def externalise_ids(html: str, row: CsvItem) -> str:
    html = html.replace('id="', 'id="' + str(row.id))
    html = html.replace('href="#', f'href="#{row.id}')
    return html

def flatten_subcontents(soup: Soup):
    for tag in soup.select("div.table_of_contents"):
        tag["class"] = [# type: ignore
            "table_of_contents",
            "standalone",
            "well"
        ]
        add_style(tag, "padding-bottom:0;")

def remove_border_radius(soup: Soup):
    removals = [
        "div.redbox",
        "div.greenbox",
        "div.graybox",
        "div.yellowbox",
        "div.bluebox",
        ".btn"
        "pre.fixed",
        "div.table_of_contents"
    ]

    for selector in removals:
        for tag in soup.select(selector):
            add_style(tag, "border-radius:0;")


def raise_from_mariadb(soup: Soup):
    """Increases padding for `mariadb starting with ...` to prevent text being sliced on page breaks"""
    for tag in soup.select("div.mariadb"):
        tag.attrs["style"] = "padding:1em;"


def add_style(tag: Tag, style: str):
    if "style" not in tag.attrs:
        tag["style"] = style
    elif isinstance(tag.attrs["style"], str):
        tag["style"] = [tag["style"], style] # type: ignore
    else:
        tag["style"].append(style) # type: ignore


def remove_numbered_headers(soup: Soup, row: CsvItem):
    for header in soup.select("h2,h3,h4,h5,h6"):
        if not header.text:
            return
        if header.text[0].isnumeric():
            header.string = "- " + header.text