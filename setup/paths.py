from setup.logger import log
import requests

def get_html(url: str, port: int) -> str | None:
    return get_url("kb/" + url, port)

def error(msg: str):
    log.error(msg)
    exit(1)

def get_url(url: str, port: int) -> str | None:
    try:
        response = requests.get(f"http://127.0.0.1:{port}/{url}")
    except requests.ConnectionError:
        return None
    if response.status_code != 200:
        return None
    return response.text