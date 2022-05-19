use console::style;
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

impl Data {
    pub fn display(self) {
        println!("温度: {}  湿度: {}", self.temperature, self.humidity);
        println!("pm2.5: {}  pm10: {}", self.pm25, self.pm10);
        println!("空气质量: {}", self.quality);
    }
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

impl CityInfo {
    pub fn display(self) {
        println!("城市: {}", self.city);
    }
}

#[derive(Deserialize, Debug)]
struct Weather {
    message: String,
    status: i32,
    date: String,
    time: String,
    #[serde(rename="cityInfo")]
    city: CityInfo,
    data: Data
}

impl Weather {
    pub fn display(self) {
        if self.status != 200 {
            println!("{}", style("Query weather failed").red());
        } else {
            self.city.display();
            self.data.display();
        }
    }
}
    
pub fn get_weather(city: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path: String = URL.to_owned() + city;

    let context = reqwest::blocking::get(path)?.text()?;

    let weather: Weather = serde_json::from_str(&context)?;
    weather.display();

    Ok(())
}
