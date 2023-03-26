from src.version import Version
def test_version_from_str():
    assert Version.from_str("103") == Version(10, 3)
    assert Version.from_str("1003") == Version(10, 3)

def test_version_cmp():
    # makes sure version comparisons are equal to tuple comparisons
    combos = [(major, minor) for major in (10, 11) for minor in (0, 4)]
    for i in combos:
        for j in combos:
            assert (i > j) == (Version(*i) > Version(*j))
            assert (i >= j) == (Version(*i) >= Version(*j))
            assert (i < j) == (Version(*i) < Version(*j))
            assert (i <= j) == (Version(*i) <= Version(*j))
            assert (i == j) == (Version(*i) == Version(*j))