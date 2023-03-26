from bs4 import BeautifulSoup as Soup
from src import tag_rules
def test_paragraphTag_():
    tag = Soup(
"""    <p>testing... testing...  

testing... testing...
testing...
testing...</p>
""", features="lxml")
    tag_rules.paragraphTag(tag)
    output = tag.string
    expected_output = "testing... testing... testing... testing... testing... testing..."
    assert output is not None
    assert output.strip() == expected_output

def test_headerTag():
    base: str = "<h0>Header0</h0>"

    for i in range(1, 7):
        string = base.replace('0', str(i))

        tag = Soup(string, features="lxml")
        tag_rules.headerTag(tag)

        output = tag.string
        expected_output = f"\nHeader{i}\n-------\n"

        assert output == expected_output

def test_codeTag():
    tag: Soup = Soup(
        "    <code>code</code>",
        features="lxml")

    tag_rules.codeTag(tag)
    output = tag.string

    expected_output = "\n\ncode\n"
    assert output == expected_output

def test_listTag():
    tag: Soup = Soup(
        "    <li>Point One</li>",
        features="lxml"
        )
    tag_rules.listTag(tag)
    output = tag.string

    expected_output = "* Point One"
    assert output == expected_output

def test_tableTag():
    #TODO
    pass
