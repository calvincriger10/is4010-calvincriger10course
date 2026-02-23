import json

def save_contacts_to_json(contacts, filename):
    """
    Saves a list of contacts (dictionaries) to a file in JSON format.
    """
    with open(filename, 'w', encoding='utf-8') as f:
        json.dump(contacts, f, indent=4)

def load_contacts_from_json(filename):
    """
    Loads a list of contacts from a JSON file.

    Returns an empty list if the file does not exist.
    """
    try:
        with open(filename, 'r', encoding='utf-8') as f:
            return json.load(f)
    except FileNotFoundError:
        return []

if __name__ == '__main__':
    contacts_file = 'lab07/contacts.json'

    my_contacts = load_contacts_from_json(contacts_file)
    print(f"Loaded {len(my_contacts)} contact(s).")

    new_contact = {"name": "Charles Babbage", "email": "charles@computers.org"}
    my_contacts.append(new_contact)
    print(f"Added a new contact for {new_contact['name']}.")

    save_contacts_to_json(my_contacts, contacts_file)
    print("Saved contacts to disk.")