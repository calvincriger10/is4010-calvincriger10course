# lab06.py

class Book:
    """A class that represents a physical book."""

    def __init__(self, title, author, year):
        self.title = title
        self.author = author
        self.year = year

    def __str__(self):
        return f'"{self.title}" by {self.author} ({self.year})'

    def get_age(self):
        current_year = 2025
        return current_year - self.year


class EBook(Book):
    """A class that represents an electronic book."""

    def __init__(self, title, author, year, file_size):
        super().__init__(title, author, year)
        self.file_size = file_size

    def __str__(self):
        parent_str = super().__str__()
        return f"{parent_str} ({self.file_size} MB)"
    

if __name__ == "__main__":
    # Test Book
    book = Book("The Hobbit", "J.R.R. Tolkien", 1937)
    print(book)
    print("Age:", book.get_age())

    # Test EBook
    ebook = EBook("Dune", "Frank Herbert", 1965, 5)
    print(ebook)
    print("Age:", ebook.get_age())