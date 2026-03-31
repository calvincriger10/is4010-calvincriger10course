import os
import pytest

from week08.favorites import FavoritesManager


@pytest.fixture
def temp_file(tmp_path):
    return str(tmp_path / "favorites.json")


def test_add_favorite(temp_file):
    manager = FavoritesManager(temp_file)
    assert manager.add("home", "Cincinnati, OH") is True
    assert manager.get_location("home") == "Cincinnati, OH"


def test_add_duplicate_favorite(temp_file):
    manager = FavoritesManager(temp_file)
    manager.add("home", "Cincinnati, OH")
    assert manager.add("home", "Columbus, OH") is False


def test_remove_favorite(temp_file):
    manager = FavoritesManager(temp_file)
    manager.add("home", "Cincinnati, OH")
    assert manager.remove("home") is True
    assert manager.get_location("home") is None


def test_remove_nonexistent_favorite(temp_file):
    manager = FavoritesManager(temp_file)
    assert manager.remove("work") is False


def test_list_all_favorites(temp_file):
    manager = FavoritesManager(temp_file)
    manager.add("home", "Cincinnati, OH")
    manager.add("work", "Columbus, OH")
    favorites = manager.list_all()
    assert len(favorites) == 2
    assert favorites["home"] == "Cincinnati, OH"
    assert favorites["work"] == "Columbus, OH"


def test_get_location_case_insensitive(temp_file):
    manager = FavoritesManager(temp_file)
    manager.add("Home", "Cincinnati, OH")
    assert manager.get_location("home") == "Cincinnati, OH"
    assert manager.get_location("HOME") == "Cincinnati, OH"


def test_remove_case_insensitive(temp_file):
    manager = FavoritesManager(temp_file)
    manager.add("Home", "Cincinnati, OH")
    assert manager.remove("home") is True


def test_persistence_across_instances(temp_file):
    manager1 = FavoritesManager(temp_file)
    manager1.add("home", "Cincinnati, OH")

    manager2 = FavoritesManager(temp_file)
    assert manager2.get_location("home") == "Cincinnati, OH"


def test_load_corrupted_json_file(temp_file):
    with open(temp_file, "w", encoding="utf-8") as file:
        file.write("{bad json")

    manager = FavoritesManager(temp_file)
    assert manager.list_all() == {}


def test_load_nonexistent_file(temp_file):
    if os.path.exists(temp_file):
        os.remove(temp_file)

    manager = FavoritesManager(temp_file)
    assert manager.list_all() == {}