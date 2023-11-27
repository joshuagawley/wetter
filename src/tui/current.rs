use crate::geolocation::Location;
use crate::tui::border::{Border, Edge, Separator};
use crate::tui::dimension::{Dimensions, MIN_CELL_WIDTH, MIN_WIDTH};
use crate::tui::weather::WindDirection;
use crate::weatherkit::CurrentWeather;
use console::style;
use inflector::Inflector;

#[derive(Debug)]
pub struct PreparedCurrent<'a> {
    location: &'a str,
    condition_code: String,
    temperature: String,
    apparent_temperature: String,
    humidity: String,
    dew_point: String,
    wind: String,
    pressure: String,
    dimensions: Dimensions,
}

impl CurrentWeather {
    pub fn prepare(self, location: &Location) -> anyhow::Result<PreparedCurrent<'_>> {
        let temperature = format!("{:.1}ºC", self.temperature);
        let apparent_temperature = format!("{:.1}ºC", self.temperature_apparent);
        let humidity = format!("Humidity: {}%", self.humidity * 100.0);
        let dew_point = format!("Dew point: {:.1}ºC", self.temperature_dew_point);
        let maybe_wind_direction = match self.wind_direction {
            Some(wd) => Some(WindDirection::get_direction(wd)?),
            None => None,
        };

        let wind = match maybe_wind_direction {
            Some(wind_direction) => format!(
                "{} {:.1}kph {}",
                wind_direction.get_icon(),
                self.wind_speed,
                wind_direction
            ),

            None => format!("{:.1}km/h", self.wind_speed),
        };

        let pressure = format!("{}hPa", self.pressure);

        let title_padding = 2 * 2;
        let longest_cell_width = humidity.len();
        let term_width = MIN_WIDTH + title_padding;
        let cell_width = if longest_cell_width > MIN_CELL_WIDTH {
            longest_cell_width
        } else {
            MIN_CELL_WIDTH + 2
        };

        let dimensions = Dimensions::new(term_width, cell_width);

        Ok(PreparedCurrent {
            location: &location.city,
            condition_code: self.condition_code.to_title_case(),
            temperature,
            apparent_temperature,
            humidity,
            dew_point,
            wind,
            pressure,
            dimensions,
        })
    }
}

impl PreparedCurrent<'_> {
    pub fn render(self) {
        let (term_width, cell_width) = self.dimensions.into();

        // Border Top
        println!("{}", Edge::Top.fmt(term_width));

        // Address
        println!(
            "{} {: ^width$} {}",
            Border::Left.fmt(),
            style(self.location).bold(),
            Border::Right.fmt(),
            width = term_width - 2
        );

        // Separator
        println!("{}", Separator::Single.fmt(term_width));

        // Temperature and condition code
        println!(
            "{} {: <width$} {}",
            Border::Left.fmt(),
            style(format!("{}, {}", self.condition_code, self.temperature)).bold(),
            Border::Right.fmt(),
            width = term_width - 2,
        );

        // Apparent temperature
        println!(
            "{} {: <width$} {}",
            Border::Left.fmt(),
            format!("Feels like {:.1}ºC", self.apparent_temperature),
            Border::Right.fmt(),
            width = term_width - 2
        );

        // Blank line
        println!("{}", Separator::Blank.fmt(term_width));

        // Humidity and dew point
        println!(
            "{} {: <width$} {}",
            Border::Left.fmt(),
            format!(
                "{: <cell_width$} {}",
                self.humidity,
                self.dew_point,
                cell_width = cell_width - 1
            ),
            Border::Right.fmt(),
            width = term_width - 2
        );

        // Wind and pressure
        println!(
            "{} {: <cell_width$}{: <width$} {}",
            Border::Left.fmt(),
            self.wind,
            self.pressure,
            Border::Right.fmt(),
            width = term_width - 2 - cell_width,
        );

        println!("{}", Edge::Bottom.fmt(term_width));
    }
}
