from pathlib import Path

from pdf.generate_pdf import generate_full_pdf
from setup.config import read_config
from setup.kb_urls import read_csv
from setup.languages import read_languages


def main():
    config = read_config()
    csv = read_csv(config.num_rows, config.port)
    language_csvs = read_languages(csv, config)
    try:
        for lang, lang_csv in language_csvs.items():
            generate_full_pdf(lang_csv, Path(f"output_{lang}"), config)
    finally:
        Path(config.wkhtml_settings["dump-outline"]).unlink(missing_ok=True)


if __name__ == "__main__":
    main()

