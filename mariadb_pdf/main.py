import logging
from pathlib import Path

from pdf.generate_pdf import generate_full_pdf
from setup.config import read_config
from setup.kb_urls import read_csv
from setup.languages import read_languages

log = logging.getLogger(__name__)


def main():
    config = read_config()
    csv = read_csv(config.num_rows, config.port)
    language_csvs = read_languages(csv, config)
    logging.basicConfig(level=logging.DEBUG)
    try:
        for lang, lang_csv in language_csvs.items():
            log.info(f"Generating {lang}({len(lang_csv)})")
            generate_full_pdf(lang_csv, Path(f"output_{lang}"), config)
    finally:
        Path(config.wkhtml_settings["dump-outline"]).unlink(missing_ok=True)
    log.info("Finished")


if __name__ == "__main__":
    main()
