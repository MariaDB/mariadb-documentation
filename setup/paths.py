from pathlib import Path
import requests
DIR_PATH_STR ="../kb_archive/HTML" 
DIR_PATH = Path(DIR_PATH_STR)
BASE_KB = "https://mariadb.com/kb/"
URL_LOCATIONS_PATH = Path("../url_locations.txt")
PORT = 7032

def get_html(url: str) -> str | None:
    response = requests.get(f"http://127.0.0.1:{PORT}/{url}")
    if response.status_code == 404:
        return None
    return response.text