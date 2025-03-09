# Weather Station

This proyect is forked from [weatherApplicationCLI](https://github.com/BekBrace/weatherApplicationCLI) by [BekBrace](https://github.com/BekBrace)

This repository contains a Rust program that fetches and displays weather information from the OpenWeatherMap API based on command args for city name and country code.

-image-

## Instalation and usage

You can install it by binaries or a clone

```bash
git clone https://github.com/Fidnix/weather-cli-app.git
cargo build
cargo install --path .
weather --city Lima --country-code PE
```

This CLI only has two arguments:
* city
* contry code

## Installation for development

Clone the project

```bash
git clone https://github.com/Fidnix/weather-cli-app.git
```

Create an account in [OpenWeather](https://openweathermap.org/) and generate an API Key to generate requests, save the key into an `.env` file and run the project:

```bash
cargo run -- --city Lima --country-code PE
```

> [!IMPORTANT] API Key
> Don't forget the API Key

## Dependencies
* serde - A Rust library for serializing and deserializing data structures
* colored - A Rust library for terminal text coloring
* reqwest - A Rust library for making HTTP requests
