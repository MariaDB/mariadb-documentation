"""Version selecting and debug info, interface to run generation script"""
import time
import os
from pathlib import Path

import argparse
from dataclasses import dataclass

from src.generate_help_table import generate_help_table
from src import debug
from src.version import Version


# Preparing system for colored text
os.system('')

OUTPUT_FILENAME: str = "fill_help_tables{}.sql"
DEFAULT_CONCAT_SIZE = 15000

@dataclass(slots=True)
class Args:
    versions: list[Version]
    quiet: bool
    concat_size: int
    output_path: Path
    port: int

    @classmethod
    def parse(cls, args = None):
        parser = argparse.ArgumentParser()
        parser.add_argument("-q", "--quiet", action="store_true", help="Quiet/Hide Progress Bar")
        parser.add_argument("-l", "--length", type=int, default=DEFAULT_CONCAT_SIZE)
        parser.add_argument("-v", "--versions", "--version", nargs="+", required=True)
        parser.add_argument("-p", "--port", type=int, default=7032)
        parser.add_argument("-o", "--output", default="./output/")

        args = parser.parse_args(args)
        versions = read_versions(args.versions)
        return cls(versions, args.quiet, args.length, Path(args.output), args.port)

# Functions
def read_versions(args: list[str]) -> list[Version]:
    """Reads the version number while giving precise debug info"""
    versions = []
    for version_str in args:
        assert version_str.isnumeric(), version_str
        assert len(version_str) >= 3
        version = Version.from_str(version_str)
        assert version.major >= 10
        versions.append(version)
    return versions

def check_max_char_length(sql: str, concat_size: int):
    max_line_length = 0
    lines = sql.splitlines()
    for index, line in enumerate(lines, 1):
        max_line_length = max(max_line_length, len(line))
        if len(line) > concat_size:
            debug.warn(f"Line {index} above {concat_size}: ({len(line)})")
        if line.replace("\\'", "").count("'") % 2 != 0:
            debug.warn(f"Line {index} uneven number of single quotes")
        
    debug.info(f"Number of Lines: {len(lines)}")
    debug.info(f"Max Line Length: {max_line_length}")

def main(args = None) -> int:
    start = time.perf_counter()
    args = Args.parse(args)
    debug.success(f"Selected Versions: {args.versions}")

    args.output_path.mkdir(exist_ok=True)
    for version in args.versions:
        print()
        debug.success(f"Generating Version: {version}")
        output_filepath = Path(args.output_path / OUTPUT_FILENAME.format(f"{version.major}{version.minor}"))
        output = generate_help_table(version, args.concat_size-400, args.port, not args.quiet)
        check_max_char_length(output, args.concat_size)
        Path(output_filepath).write_text(output, encoding="utf-8")
    time_taken = time.perf_counter() - start
    debug.time_info(f"Took {time_taken:.2f}s")
    return 0

if __name__ == "__main__":
    exit(main())