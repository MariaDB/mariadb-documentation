from src.generate_help_table import (
    get_help_topic_text, get_name,
    insert_help_keyword
)

def test_get_help_topic_text():
    output = get_help_topic_text('1','2','3','4','5','6')

    expected_output = "insert into help_topic (help_topic_id,help_category_id,name,description,example,url) values "
    expected_output += f"(1,2,'3','4','5','6');"
    
    assert output == expected_output

def test_get_name():
    urls_to_names = [
        ("https://mariadb.com/kb/en/select/", "select"),
        ("mariadb.com/kb/en/alter-user/", "alter-user"),
        ("https://mariadb.com/kb/en/area", "area"),
        ("mariadb.com/kb/en/alter-table", "alter-table"),
    ]

    for url, expected_name in urls_to_names:
        outputed_name = get_name(url)
        assert outputed_name == expected_name


def test_insert_help_keyword():
    
    expected_output = "insert into help_keyword values (12, 'keyword');"
    output = insert_help_keyword(12, "keyword")
    
    assert expected_output == output 

def test_insert_help_relations():
    from src.generate_help_table import insert_help_relations

    expected_output = "insert into help_relation values (1, 2);"
    output = insert_help_relations(1, 2)
    
    assert expected_output == output 