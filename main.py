"""Version selecting and debug info, interface to run generation script"""
import time
import os
from pathlib import Path
from src.generate_help_table import generate_help_table
import src.debug as debug
from src.version import Version
import argparse

# Preparing system for colored text
os.system('')

OUTPUT_PATH: str = "output/fill_help_tables{}.sql"
DEFAULT_CONCAT_SIZE = 15000

def parse_args(args = None) -> tuple[list[Version], int]:
    parser = argparse.ArgumentParser()
    parser.add_argument("--length", "-l", type=int, default=DEFAULT_CONCAT_SIZE)
    parser.add_argument("--versions", "--version", "-v", nargs="+", required=True)
    args = parser.parse_args(args)

    versions = read_versions(args.versions)
    return versions, args.length

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
    versions, concat_size = parse_args(args)
    debug.success(f"Selected Versions: {versions}")

    Path("output").mkdir(exist_ok=True)
    for version in versions:
        print()
        debug.success(f"Generating Version: {version}")
        output_filepath = Path(OUTPUT_PATH.format(f"{version.major}{version.minor}"))
        output = generate_help_table(version, concat_size-400)
        check_max_char_length(output, concat_size)
        Path(output_filepath).write_text(output, encoding="utf-8")
    time_taken = time.perf_counter() - start
    debug.time_info(f"Took {time_taken:.2f}s")
    return 0

if __name__ == "__main__":
    exit(main())