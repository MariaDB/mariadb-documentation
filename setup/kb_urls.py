from .paths import PORT
from .logger import log

from typing import Iterable
from pathlib import Path
from dataclasses import dataclass

import csv
import requests

@dataclass
class CsvItem:
    header: str
    url: str
    id: str
    slugs: list[str]
    include: int
    depth: int
    depth_str: str = ""

    @classmethod
    def from_dict(cls, row: dict[str, str]):
        url: str = row["URL"]
        id = url
        slugs: list[str]
        include: int
        depth: int
        header: str = row["Header"]
        
        try:
            include = int(row["Include"]) if row["Include"].strip() != "" else 0
        except ValueError as s:
            log.error(f"Could not convert 'Include' field to integer for {url}: {s}")
            exit(1)

        if row["Depth"] == "":
            depth = 0
        elif row["Depth"].isnumeric():
            depth = int(row["Depth"])
        else:
            log.error(f"Invalid Depth Argument: {row['Depth']}")
            exit(1)

        slugs = [f"https://mariadb.com/kb/en/{slug}/" for slug in row["Duplicate slugs"].split(";") if slug.strip()]

        return cls(
            url=url,
            id=id,
            slugs=slugs,
            include=include,
            depth=depth,
            header=header,
        )

def read_csv(num_rows: int) -> list[CsvItem]:
    string = requests.get(f"http://127.0.0.1:{PORT}/kb_urls.csv").text
    content = csv.DictReader(string.splitlines())
    rows = [row for row in content if row["Include"] not in ["", "0"]]
    rows = rows[:num_rows] if num_rows > 0 else rows
    rows =  [CsvItem.from_dict(row) for row in rows if row["Include"] not in ["", "0"]]
    return apply_depth(rows)

def apply_depth(kb_urls: list[CsvItem]) -> list[CsvItem]:
    depths = []
    for row in kb_urls:
        if row.depth >= len(depths):
            depths.extend([0] * (row.depth-len(depths)))
        elif row.depth < len(depths):
            for _ in range(len(depths)-row.depth):
                depths.pop()
        depths[row.depth-1] += 1
        row.depth_str = '.'.join([str(num) for num in depths])
        while row.depth_str.startswith("0."):
            row.depth_str = row.depth_str.removeprefix("0.")
    return kb_urls
