# lab05.py

users = [
    {"name": "alice", "age": 30, "is_active": True, "email": "alice@example.com"},
    {"name": "bob", "age": 25, "is_active": False},
    {"name": "charlie", "age": 35, "is_active": True, "email": "charlie@example.com"},
    {"name": "david", "age": "unknown", "is_active": False},
]


def calculate_average_age(users):
    total_age = 0
    count = 0

    try:
        for user in users:
            age = user.get("age")
            if isinstance(age, int):
                total_age += age
                count += 1

        return total_age / count

    except ZeroDivisionError:
        print("error: cannot calculate average age.")
        return 0.0


def get_active_user_emails(users):
    emails = []

    try:
        for user in users:
            if user.get("is_active") and user.get("email"):
                emails.append(user["email"])

        return emails

    except KeyError:
        print("error: missing key.")
        return []


if __name__ == "__main__":
    avg_age = calculate_average_age(users)
    print(avg_age)

    emails = get_active_user_emails(users)
    print(emails)
