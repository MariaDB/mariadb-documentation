from setup.config import TocConfig, TocTypeConfig
from .process_html_page import PAGE_BREAK
from dataclasses import dataclass

@dataclass(slots=True)
class TocItem:
    page_num: int
    header: str
    link_id: str

def create_contents(outline: list[TocItem], config: TocConfig) -> str:
    header_style = 'style="font-size: 30px; font-weight: bold;text-align:center;"'    
    toc_header = f'<p {header_style}>''{}</p>'
    main_contents = [
        toc_header.format("Table of Contents"),
        '<ul style="padding-left:0;">'
    ]
    chapter_contents = [
        toc_header.format("Chapter Contents"),
        '<ul style="padding-left:0;">'
    ]

    main_ul = f'<ul style="list-style:none;padding-left:{config.main.padding_left};">'
    chapter_ul = f'<ul style="list-style:none;padding-left:{config.chapter.padding_left};">'
    prev_depth = 1
    prev_chapter_depth = 1
    max_chapter_depth = 2
    for row in outline:
        depth_str: str = row.header.strip().split(' ')[0]
        depth: int = depth_str.count('.')+1
        if depth > prev_depth:
            main_contents.extend([main_ul] * (depth-prev_depth))
        elif depth < prev_depth:
            main_contents.extend(["</ul>"] * (prev_depth-depth))
        if depth <= max_chapter_depth:
            if depth > prev_chapter_depth:
                chapter_contents.extend([chapter_ul] * (depth-prev_chapter_depth))
            elif depth < prev_chapter_depth:
                chapter_contents.extend(["</ul>"] * (prev_chapter_depth-depth))
        # Text
        text = row.header
        if depth == 1:
            text = "Chapter " + text
        item = create_item(text, row.page_num, row.link_id, config.main)
        main_contents.append(item)
        prev_depth = depth
        if depth <= max_chapter_depth:
            item = create_item(text, row.page_num, row.link_id, config.chapter)
            chapter_contents.append(item)
            prev_chapter_depth = depth
    main_contents.extend(["</ul>"] * prev_depth)
    chapter_contents.extend(["</ul>"] * max(prev_chapter_depth, max_chapter_depth))
    
    chapter_contents = "\n".join(chapter_contents)
    main_contents = "\n".join(main_contents)

    assert chapter_contents.count("<ul") == chapter_contents.count("</ul")
    assert main_contents.count("<ul") == main_contents.count("</ul")

    return chapter_contents + PAGE_BREAK + main_contents + PAGE_BREAK


def create_item(text: str, page_num: int, link_id: str, config: TocTypeConfig) -> str:
    """ Returns a div with all the information styled correctly, div is not contained within a 'li' tag"""
    href = f'href="#{link_id}"'
    div_attrs = 'class="pdfhorizontal_dotted_line"'
    # Text Tag
    a_style = f'text-decoration: none; color: black; background: #ffffff; font-size: {config.font_size};'
    if text.startswith("Chapter"):
        a_style += ' font-weight: bold;'
    text_tag = f'<span><a style="{a_style}" {href}>{text}</a></span>'
    # Page Tag
    page_style = 'style="float: right; color: black; background: #ffffff;"'
    page_tag = f'<a {href} {page_style}>{page_num}</a>'
    # Final Tag
    return f'<li><div {div_attrs}>{text_tag}{page_tag}</div></span></li>'
