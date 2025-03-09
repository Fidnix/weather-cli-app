use serde::Deserialize; // Importing serde for JSON deserialization
use colored::*; // Importing colored crate for text coloring

// Struct to deserialize the JSON response from OpenWeatherMap API
#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    weather: Vec<Weather>, // Contains weather information
    main: Main, // Contains main weather parameters
    wind: Wind, // Contains wind information
    name: String, // Contains the name of the queried location
}

// Struct to represent weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String, // Contains textual weather description
}

// Struct to represent main weather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64, // Temperature in Celsius
    humidity: f64, // Humidity in percentage
    pressure: f64, // Atmospheric pressure in hPa
}

// Struct to represent wind information
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64, // Wind speed in meters per second
}

// Methods implementation for WeatherResponse abstraction 
impl WeatherResponse {
    // Function to get weather information from OpenWeatherMap API
    pub fn get_info(city: &str, country_code: &str, api_key: &str) -> Result<Self, reqwest::Error> {
        // Constructing the URL for API request
        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
            city, country_code, api_key
        );
    
        // Sending a blocking GET request to the API endpoint
        let response = reqwest::blocking::get(&url)?;
        // Parsing the JSON response into WeatherResponse struct
        let response_json = response.json::<WeatherResponse>()?;
        Ok(response_json) // Returning the deserialized response
    }
    
    // Function to display weather information
    pub fn display_info(&self) {
        // Extracting weather information from the response
        let name = &self.name;
        let description = &self.weather[0].description;
        let temperature = self.main.temp;
        let humidity = self.main.humidity;
        let pressure = self.main.pressure;
        let wind_speed = self.wind.speed;
    
        // Formatting weather information into a string
        let weather_text = format!(
"Weather in {}: {} {}
> Temperature: {:.1}°C, 
> Humidity: {:.1}%, 
> Pressure: {:.1} hPa, 
> Wind Speed: {:.1} m/s",
        name,
        description,
        get_temperature_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );
    
        // Coloring the weather text based on weather conditions
        let weather_text_colored = match description.as_str() {
            "clear sky" => weather_text.bright_yellow(),
            "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
            "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
            "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
            _ => weather_text.normal(),
        };
    
        // Printing the colored weather information
        println!("{}", weather_text_colored);
    }
}

// Function to get emoji based on temperature
fn get_temperature_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}