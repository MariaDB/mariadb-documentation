from setup.paths import url_to_path, format_url, BASE_KB, DIR_PATH

def test_format_url_bulk():
    inputs_and_outputs = (DIR_PATH / "../url_tests.txt").read_text(encoding="utf-8").split("\n")

    for content in inputs_and_outputs:
        input, output = content.split(" ")
        assert format_url(input.strip()) == output.strip()

# test url_to_path
def test_url_to_path_en():
    url = BASE_KB + "en/"
    expected = DIR_PATH / "en.html/"
    assert url_to_path(DIR_PATH, url) == expected

def test_url_to_path_select():
    url = BASE_KB + "en/select/"
    expected = DIR_PATH / "en/select.html/"
    assert url_to_path(DIR_PATH, url) == expected

def test_url_to_path_source():
    url = BASE_KB + "en/alter-user/+source/"
    expected = DIR_PATH / "en/alter-user/+source.html/"
    assert url_to_path(DIR_PATH, url) == expected
