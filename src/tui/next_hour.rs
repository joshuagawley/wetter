use crate::geolocation::Location;
use crate::weatherkit::NextHourForecast;

struct PreparedNextHourSummary {}

impl PreparedNextHourSummary {
    fn render(&self) -> String {
        "".to_owned()
    }
}

pub struct PreparedNextHourSummaries {
    location: String,
    summaries: Vec<PreparedNextHourSummary>,
    width: usize,
}

impl NextHourForecast {
    pub fn prepare(self, location: &Location) -> PreparedNextHourSummaries {
        let summaries = Vec::new();
        let width = summaries.len();
        PreparedNextHourSummaries {
            location: location.to_string(),
            summaries,
            width,
        }
    }
}

impl PreparedNextHourSummaries {
    pub fn render(self) {
        while let Some(summary) = self.summaries.iter().next() {
            summary.render();
        }
    }
}
