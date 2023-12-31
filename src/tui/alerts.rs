use crate::geolocation::Location;
use crate::weatherkit::WeatherAlertsCollection;

struct PreparedWeatherAlertSummary {}

pub struct PreparedWeatherAlertSummaries {
    location: String,
    summaries: Vec<PreparedWeatherAlertSummary>,
    details_url: String,
    width: usize,
}

impl WeatherAlertsCollection {
    pub fn prepare(self, location: &Location) -> PreparedWeatherAlertSummaries {
        let summaries = Vec::new();
        let details_url = self.details_url.unwrap_or("".to_owned());
        let width = 0;

        PreparedWeatherAlertSummaries {
            location: location.to_string(),
            summaries,
            details_url,
            width,
        }
    }
}

impl PreparedWeatherAlertSummaries {
    pub fn render(self) {}
}
