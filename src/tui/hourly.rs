use crate::geolocation::Location;
use crate::weatherkit::HourlyForecast;

struct PreparedHourlySummary {}

pub struct PreparedHourlySummeries {
    location: String,
    summaries: Vec<PreparedHourlySummary>,
    width: usize,
}

impl HourlyForecast {
    pub fn prepare(self, location: &Location) -> PreparedHourlySummeries {
        let summaries = Vec::new();
        PreparedHourlySummeries {
            location: location.to_string(),
            summaries,
            width: 0,
        }
    }
}

impl PreparedHourlySummeries {
    pub fn render(self) {}
}
