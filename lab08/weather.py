import argparse

from lab08.favorites import FavoritesManager
from lab08.weather_api import WeatherAPI, format_current_weather, format_forecast
from lab08.config import WEATHER_API_KEY, WEATHER_API_BASE_URL


def resolve_location(location, favorites_manager):
    favorite_location = favorites_manager.get_location(location)
    if favorite_location:
        return favorite_location
    return location


def handle_current(args, api, favorites_manager):
    location = resolve_location(args.location, favorites_manager)
    data = api.get_current_weather(location)
    print(format_current_weather(data))


def handle_forecast(args, api, favorites_manager):
    location = resolve_location(args.location, favorites_manager)
    data = api.get_forecast(location, args.days)
    print(format_forecast(data))


def handle_favorites_add(args, favorites_manager):
    if favorites_manager.add(args.name, args.location):
        print(f"Added favorite '{args.name}' -> {args.location}")
    else:
        print(f"Favorite '{args.name}' already exists.")


def handle_favorites_list(favorites_manager):
    favorites = favorites_manager.list_all()
    if not favorites:
        print("No favorites saved.")
        return

    print("Saved favorites:")
    for name, location in favorites.items():
        print(f"- {name}: {location}")


def handle_favorites_remove(args, favorites_manager):
    if favorites_manager.remove(args.name):
        print(f"Removed favorite '{args.name}'.")
    else:
        print(f"Favorite '{args.name}' not found.")


def create_parser():
    parser = argparse.ArgumentParser(description="Weather CLI Application")
    subparsers = parser.add_subparsers(dest="command")

    current_parser = subparsers.add_parser("current", help="Get current weather")
    current_parser.add_argument("location", help="Location or favorite name")

    forecast_parser = subparsers.add_parser("forecast", help="Get weather forecast")
    forecast_parser.add_argument("location", help="Location or favorite name")
    forecast_parser.add_argument(
        "--days",
        type=int,
        choices=[1, 2, 3],
        default=3,
        help="Number of forecast days (1-3)"
    )

    favorites_parser = subparsers.add_parser("favorites", help="Manage favorite locations")
    favorites_subparsers = favorites_parser.add_subparsers(dest="favorites_command")

    add_parser = favorites_subparsers.add_parser("add", help="Add a favorite")
    add_parser.add_argument("name", help="Favorite nickname")
    add_parser.add_argument("location", help="Location string")

    favorites_subparsers.add_parser("list", help="List all favorites")

    remove_parser = favorites_subparsers.add_parser("remove", help="Remove a favorite")
    remove_parser.add_argument("name", help="Favorite nickname")

    return parser


def main():
    parser = create_parser()
    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        return

    favorites_manager = FavoritesManager()
    api = WeatherAPI(WEATHER_API_KEY, WEATHER_API_BASE_URL)

    if args.command == "current":
        handle_current(args, api, favorites_manager)
    elif args.command == "forecast":
        handle_forecast(args, api, favorites_manager)
    elif args.command == "favorites":
        if args.favorites_command == "add":
            handle_favorites_add(args, favorites_manager)
        elif args.favorites_command == "list":
            handle_favorites_list(favorites_manager)
        elif args.favorites_command == "remove":
            handle_favorites_remove(args, favorites_manager)
        else:
            parser.print_help()


if __name__ == "__main__":
    main()