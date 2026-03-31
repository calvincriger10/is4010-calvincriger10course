import json
import os


class FavoritesManager:
    def __init__(self, filename="week08/favorites.json"):
        self.filename = filename
        self.favorites = self._load()

    def _load(self):
        if not os.path.exists(self.filename):
            return {}

        try:
            with open(self.filename, "r", encoding="utf-8") as file:
                data = json.load(file)
                if isinstance(data, dict):
                    return data
                return {}
        except (json.JSONDecodeError, OSError):
            return {}

    def save(self):
        try:
            with open(self.filename, "w", encoding="utf-8") as file:
                json.dump(self.favorites, file, indent=4)
        except OSError:
            print("Error: Could not save favorites.")

    def add(self, name, location):
        for existing_name in self.favorites:
            if existing_name.lower() == name.lower():
                return False

        self.favorites[name] = location
        self.save()
        return True

    def remove(self, name):
        for existing_name in list(self.favorites.keys()):
            if existing_name.lower() == name.lower():
                del self.favorites[existing_name]
                self.save()
                return True
        return False

    def list_all(self):
        return self.favorites

    def get_location(self, name):
        for existing_name, location in self.favorites.items():
            if existing_name.lower() == name.lower():
                return location
        return None