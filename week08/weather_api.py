import requests


class WeatherAPI:
    def __init__(self, api_key, base_url="http://api.weatherapi.com/v1"):
        self.api_key = api_key
        self.base_url = base_url

    def get_current_weather(self, location):
        url = f"{self.base_url}/current.json"
        params = {
            "key": self.api_key,
            "q": location
        }

        try:
            response = requests.get(url, params=params, timeout=10)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException:
            return None

    def get_forecast(self, location, days=3):
        url = f"{self.base_url}/forecast.json"
        params = {
            "key": self.api_key,
            "q": location,
            "days": days
        }

        try:
            response = requests.get(url, params=params, timeout=10)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException:
            return None


def format_current_weather(data):
    if not data:
        return "Error: Unable to fetch current weather."

    location = data["location"]["name"]
    country = data["location"]["country"]
    current = data["current"]

    return (
        "=" * 50 + "\n"
        f"Current Weather for {location}, {country}\n"
        + "=" * 50 + "\n"
        f"Condition: {current['condition']['text']}\n"
        f"Temperature: {current['temp_f']}°F ({current['temp_c']}°C)\n"
        f"Feels Like: {current['feelslike_f']}°F ({current['feelslike_c']}°C)\n"
        f"Humidity: {current['humidity']}%\n"
        f"Wind: {current['wind_mph']} mph {current['wind_dir']}\n"
        f"Last Updated: {current['last_updated']}\n"
        + "=" * 50
    )


def format_forecast(data):
    if not data:
        return "Error: Unable to fetch forecast."

    location = data["location"]["name"]
    country = data["location"]["country"]
    forecast_days = data["forecast"]["forecastday"]

    lines = [
        "=" * 50,
        f"Forecast for {location}, {country}",
        "=" * 50
    ]

    for day in forecast_days:
        lines.append(f"Date: {day['date']}")
        lines.append(f"Condition: {day['day']['condition']['text']}")
        lines.append(
            f"High: {day['day']['maxtemp_f']}°F ({day['day']['maxtemp_c']}°C)"
        )
        lines.append(
            f"Low: {day['day']['mintemp_f']}°F ({day['day']['mintemp_c']}°C)"
        )
        lines.append(f"Humidity: {day['day']['avghumidity']}%")
        lines.append("-" * 50)

    return "\n".join(lines)