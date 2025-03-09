use colored::*; // Importing colored crate for text coloring
use dotenvy::dotenv;
use std::env;

use clap::Parser;

use weather::WeatherResponse;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short='c', long, help="City name. e.g Paris")]
    city: String,

    #[arg(short='d', long, help="Country code. e.g FR (France)")]
    country_code: String,
}

fn main() {
    dotenv().ok();
    let args = Args::parse();

    println!("{}", "Welcome to Weather Station!".bright_yellow()); // Displaying welcome message

    // Get your API key from OpenWeatherMap
    let api_key = env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY isn't defined");

    // Calling the function to fetch weather information
    match WeatherResponse::get_info(&args.city, &args.country_code, &api_key) {
        Ok(response) => {
            WeatherResponse::display_info(&response); // Displaying weather information
        }
        Err(err) => {
            eprintln!("Error: {}", err); // Printing error message in case of failure
        }
    }

    println!("{}", "Thank you for using our software!".bright_magenta());
}
