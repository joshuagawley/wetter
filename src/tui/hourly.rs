use crate::geolocation::Location;
use crate::weatherkit::HourlyForecast;

struct PreparedHourlySummary {}

pub struct PreparedHourlySummeries<'a> {
    location: &'a str,
    summaries: Vec<PreparedHourlySummary>,
    width: usize,
}

impl HourlyForecast {
    pub fn prepare(self, location: &Location) -> PreparedHourlySummeries {
        let summaries = Vec::new();
        PreparedHourlySummeries {
            location: &location.city,
            summaries,
            width: 0,
        }
    }
}

impl PreparedHourlySummeries<'_> {
    pub fn render(self) {}
}
