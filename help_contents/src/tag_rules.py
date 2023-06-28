#imports
from bs4 import BeautifulSoup as Soup
import re
#annoying
from .format_to_text import LINE_LIMIT
from . import debug
#functions
def paragraphTag(tag: Soup):
    string = tag.text.strip().replace("\n", " ")
    string = re.sub(" +", " ", string)
    tag.string = string + "\n"

def headerTag(tag: Soup):
    """Modifies headers to have extra space and decoration"""
    length = len(tag.text)
    tag.string = "\n" + tag.text + "\n" + "-" * length + "\n"
    #tag.string = tag.string

def codeTag(tag: Soup):
    """Spaces code blocks to improve readability"""

    tag.string = "\n\n" + tag.text.replace("\\n", "\\\\n") + "\n"

def listTag(tag: Soup):
    """Marks <li> items with a *"""
    tag.string = "* " + tag.text

def tableTag(tag: Soup):
    """Turns html tables into text tables"""
    structured_table = create_table(tag)
    text = format_table(structured_table)
    tag.string = "\n" + text


#table stuff
def create_table(table) -> list:
    """Creates a table"""
    trs = table.find_all("tr")
    columns = []
    for tr in trs:
        columns.append([])
        ths = tr.find_all("th") + tr.find_all("td")
        for th in ths:
            text = th.get_text()
            columns[-1].append(text)

    return columns

def equalise_table(table):
    """makes sure there are no rows with less columns than other rows"""
    max_row_length = 0
    for row in table:
        row_length = len(row)
        max_row_length = max(max_row_length, row_length)

    for row in table:
        row_length = len(row)
        if row_length < max_row_length:
            row += [""] * (max_row_length - row_length)

    return table

def format_table(table):
    """Formats a table from a list into raw text"""
    output = ""

    equalise_table(table)
    column_widths = get_column_widths(table)
    for row in table:
        str_line = add_row_break(column_widths)
        row_lines, num_lines = get_lines(row, column_widths)
        for i in range(num_lines):
            str_line += "|"
            for index, line in enumerate(row_lines):
                str_line += " "
                if i < len(line):
                    str_line += line[i] + " " * (column_widths[index] - len(line[i]))
                else:
                    str_line += " " * column_widths[index]
                str_line += " |"
            str_line += "\n"
        output += str_line
        #test
        space_left = LINE_LIMIT-len(str_line.replace("\\'", "'").splitlines()[0])
        if space_left < 0:
            debug.warn("Table was formatted incorrectly")
    output += add_row_break(column_widths)
    return output

def get_column_widths(table: list) -> list:
    """Gets the required width for each column"""
    row: list = table[0]
    lengths: list = [len(column) for column in row]

    sum_lengths: int = sum(lengths)
    total_width: int = LINE_LIMIT - (3*len(lengths)) - 1
    #total width times the ratio of length / the sum of lengths
    column_widths = [int(total_width * (l / sum_lengths)) for l in lengths]

    return column_widths

def add_row_break(column_widths: list) -> str:
    """Breaks up rows with dashes and pluses"""
    #+-----+-----+-----+
    row_break = "+" + "".join(["-" * (width + 2) + "+" for width in column_widths])
    return row_break + "\n"

def get_lines(row: list, column_widths: list) -> tuple[(list, int)]:
    """returns the rows of lines and the number of lines for the column"""
    rows = []
    num_lines = 0
    for index, width in enumerate(column_widths):
        lines = sep_lines(row[index].strip(), width)
        rows.append(lines)
        num_lines = max(len(lines), num_lines)
    
    return rows, num_lines

#seperate lines
def sep_lines(line2: str, line_limit: int) -> list[str]:
    """Seperates the lines based on the given length"""
    lines = []
    while len(line2) > line_limit:
        line1, line2 = seperate_line(line2, line_limit)
        lines.append(line1)
    lines.append(line2)

    return lines

def seperate_line(line: str, line_limit: int) -> tuple:
    """returns two lines split based on the line limit"""
    matches = list(re.finditer(" ", line))

    for space in reversed(matches):
        index = space.start()
        #if the space is within the line limit
        if (index < line_limit):
            #split the line at that space's index
            line1 = line[:index]
            line2 = line[index + 1:]
            break
    #if no splits were made
    else:
        line1, line2 = line[:line_limit], line[line_limit+1:]
    return line1, line2