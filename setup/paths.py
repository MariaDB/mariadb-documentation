from setup.logger import log
import requests
PORT = 7032

def get_html(url: str) -> str | None:
    return get_url("kb/" + url)

def error(msg: str):
    log.error(msg)
    exit(1)

def get_url(url: str) -> str | None:
    try:
        response = requests.get(f"http://127.0.0.1:{PORT}/{url}")
    except requests.ConnectionError:
        return None
    if response.status_code != 200:
        return None
    return response.text