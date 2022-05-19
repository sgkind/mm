use serde_json;
use serde::{Deserialize};

const URL: &str = "http://t.weather.itboy.net/api/weather/city/";

#[derive(Deserialize, Debug)]
struct Day {
    date: String,
    high: String,
    low: String,
    ymd: String,
    week: String,
    sunrise: String,
    sunset: String,
    aqi: i32,
    fx: String,
    fl: String,
    #[serde(rename="type")]
    weather: String,
    notice: String,
}

#[derive(Deserialize, Debug)]
struct Data {
    #[serde(rename="shidu")]
    humidity: String,
    pm25: f64,
    pm10: f64,
    quality: String,
    #[serde(rename="wendu")]
    temperature: String,
    ganmao: String,
    forecast: Vec<Day>,
}

#[derive(Deserialize, Debug)]
struct CityInfo {
    city: String,
    #[serde(rename="citykey")]
    city_key: String,
    parent: String,
    #[serde(rename="updateTime")]
    update_time: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    message: String,
    status: i32,
    date: String,
    time: String,
    #[serde(rename="cityInfo")]
    city_info: CityInfo,
    data: Data
}
    
pub fn get_weather(city: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path: String = URL.to_owned() + city;

    let context = reqwest::blocking::get(path)?.text()?;

    let weather: Weather = serde_json::from_str(&context)?;
    println!("{:?}", weather);

    Ok(())
}
