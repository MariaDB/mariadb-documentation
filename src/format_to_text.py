#consts
LINE_LIMIT = 79
#imports
import re
from bs4 import BeautifulSoup as Soup
#annoying
from .tag_rules import *
from . import debug
#functions
def format_to_text(html: str, name: str) -> str:
    """transforms html into stripped raw text for use in documentation"""
    #clean html
    html = clean_html(html, name)
    #create soup structure
    soup = Soup(html, features="lxml")
    #modify soup
    clean_soup(soup)
    modify_tags(soup)
    remove_see_also(soup)
    #pull the text
    text = soup.get_text()
    #modify the text
    text = add_url(text, name)
    text = modify_text(text)
    
    return text

def clean_html(html: str, name: str = "") -> str:
    """Cleans up html so beautifulsoup has to do less processing"""
    #assert only one section tag
    if '<section id="content" class="limited_width col-md-8 clearfix">' not in html:
        debug.error(f"Invalid HTML for '{name}'")

    section = html.index('<section id="content" class="limited_width col-md-8 clearfix">')
    end_section = html.index('</section>')

    html = html[section: end_section + len('</section>')]
    return html

def clean_soup(soup: Soup):
    """Removes irrelevant content still left in the webpage"""
    #helper method for easy removal
    def remove(soup: Soup, *args, **kwargs):
        _ = [tag.decompose() for tag in soup.find_all(*args, **kwargs) if tag != None]

    #remove irrelevant information
    remove(soup, "div", {"id": "content_disclaimer"}) #removes a disclaimer
    remove(soup, "div", {"id": "comments"}) #remove the comments
    remove(soup, "h2", text = "Comments") #remove the comments' header
    remove(soup, "div", {"id": "subscribe"}) #removes the subscribe thingy (I don't know what this removes)
    remove(soup, "div", {"class": "simple_section_nav"}) #removes extra links
    remove(soup, "div", {"class": "table_of_contents"}) #remove side contents bar

    #remove main header
    tag = soup.find("h1")
    if tag is not None: tag.decompose() # type: ignore

def modify_tags(soup: Soup):
        """Removes information from a BeautifulSoup object"""
        #define tag rules
        tag_rules = {"p": paragraphTag,
                     "h1": headerTag, "h2": headerTag, "h3": headerTag,
                     "h4": headerTag, "h5": headerTag, "h6": headerTag,
                     "code": codeTag, "pre": codeTag,
                     "table": tableTag,
                     "li": listTag,}
        tag: Soup
        for tag in soup.descendants: # type: ignore
            if tag.name in tag_rules:
                tag_rules[tag.name](tag)
    
def remove_see_also(soup: Soup):
    """Finds the header labled 'See Also', removes it and it's next sibling"""
    for n in range(2, 7):
        see_also = soup.find(f"h{n}", {"id": "see-also", "class": "anchored_heading"})
        if see_also is not None:
            ns = see_also.find_next_sibling()
            if ns is not None: ns.decompose() # type: ignore
            see_also.decompose() # type: ignore

def modify_text(text: str) -> str:
    """Removes and modifies text"""
    text = set_line_limit(text)
    text = remove_extra_newlines(text)
    text = reduce_indents(text)
    text = modify_escape_chars(text)
    return text

def add_url(text: str, name: str):
    return text + f"\n\nURL: https://mariadb.com/kb/en/{name}/"

#modify text
def set_line_limit(text: str) -> str:
    """Assures lines do not extend past a certain length"""
    #recreate the text
    lines = []
    for line in text.split("\n"):
        lines += sep_lines(line, LINE_LIMIT) # type: ignore
    return '\n'.join(lines)

def remove_extra_newlines(text: str) -> str:
    """Replaces all groups of newlines with a max of 2"""
    return re.sub(" *\n *\n[ \n]*", "\n\n", text).strip()

def reduce_indents(text: str) -> str:
    """Halves the indent level of the text"""
    lines = text.split("\n")

    nlines = []
    for line in lines:
        #if the line has an indent
        if line.startswith("  "):
            #get the size of the indent
            spaces = re.match(" +", line)[0] # type: ignore
            #add spaces equal to half the indent to the stripped line
            line = (" " * (len(spaces) // 2)) + line.strip() 
        nlines.append(line)
    #combine the lines into new text
    output = "\n".join(nlines)
    return output

def modify_escape_chars(text: str) -> str:
    """escape character nonsense"""
    text = text.replace("\\", "\\\\") #escape escape characters

    #replace '”' and '“' with '"'
    text = text.replace("”", r'"')
    text = text.replace("“", r'"')
    #escape quotes
    text = text.replace("'", r"\'")
    text = text.replace('"', r'\"')
    #escape semicolons

    text = text.replace("\\\\\\n", "\\n")
    text = text.replace("\n", "\\n")
    return text
