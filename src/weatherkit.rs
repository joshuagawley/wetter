use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The system of units that the weather data is reported in.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UnitsSystem {
    /// Metric
    M,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub attribution_url: Option<String>,
    pub expire_time: DateTime<Utc>,
    pub language: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub provider_logo: Option<String>,
    pub provider_name: Option<String>,
    pub read_time: DateTime<Utc>,
    pub reported_time: Option<DateTime<Utc>>,
    pub temporarily_unavailable: Option<bool>,
    pub units: Option<UnitsSystem>,
    pub version: i64,
}

#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PressureTrend {
    Rising,
    Falling,
    Steady,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrentWeather {
    pub metadata: Metadata,
    pub as_of: DateTime<Utc>,
    pub cloud_cover: Option<f64>,
    pub condition_code: String,
    pub daylight: Option<bool>,
    pub humidity: f64,
    pub precipitation_intensity: f64,
    pub pressure: f64,
    pub pressure_trend: PressureTrend,
    pub temperature: f64,
    pub temperature_apparent: f64,
    pub temperature_dew_point: f64,
    pub uv_index: u8,
    pub visibility: f64,
    pub wind_direction: Option<u16>,
    pub wind_gust: Option<f64>,
    pub wind_speed: f64,
}

/// The type of precipitation forecasted to occur during the day.
#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PrecipitationType {
    /// No precipitation is occurring.
    Clear,
    /// An unknown type of precipitation is occurring.
    Precipitation,
    /// Rain or freezing rain is falling.
    Rain,
    /// Snow is falling.
    Snow,
    /// Sleet or ice pellets are falling.
    Sleet,
    /// Hail is falling
    Hail,
    /// Winter weather (wintery mix or wintery showers) is falling.
    Mixed,
}

/// THe shape of the moon as seen by the observer on the ground at a given time.
#[derive(Eq, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MoonPhase {
    /// The moon isn't visible.
    New,
    /// A crescent-shaped sliver of the moon is visible, and increasing in size.
    WaxingCrescent,
    /// Approximately half of the moon is visible, and increasing in size.,
    FirstQuarter,
    /// The entire disc of the moon is visible.
    Full,
    /// More than half of the moon is visible, and increasing in size.
    WaxingGibbous,
    /// More than half of the moon is visible, and decreasing in size.
    WaningGibbous,
    /// Approximately half of the moon is visible, and decreasing in size.
    ThirdQuarter,
    /// A crescent-shaped sliver of the moon is visible, and decreasing in size.
    WaningCrescent,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DayPartForecast {
    pub cloud_cover: f64,
    pub condition_code: String,
    pub forecast_end: DateTime<Utc>,
    pub forecast_start: DateTime<Utc>,
    pub humidity: f64,
    pub precipitation_amount: f64,
    pub precipitation_chance: f64,
    pub precipitation_type: PrecipitationType,
    pub snowfall_amount: f64,
    pub wind_direction: Option<u16>,
    pub wind_speed: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DayWeatherConditions {
    pub condition_code: String,
    pub daytime_forecast: Option<DayPartForecast>,
    pub forecast_end: DateTime<Utc>,
    pub forecast_start: DateTime<Utc>,
    pub max_uv_index: u8,
    pub moon_phase: MoonPhase,
    pub moonrise: Option<DateTime<Utc>>,
    pub moonset: Option<DateTime<Utc>>,
    pub overnight_forecast: Option<DayPartForecast>,
    pub precipitation_amount: f64,
    pub precipitation_chance: f64,
    pub precipitation_type: PrecipitationType,
    pub snowfall_amount: f64,
    pub solar_midnight: Option<DateTime<Utc>>,
    pub solar_noon: Option<DateTime<Utc>>,
    pub sunrise: Option<DateTime<Utc>>,
    pub sunrise_astronomical: Option<DateTime<Utc>>,
    pub sunrise_civil: Option<DateTime<Utc>>,
    pub sunrise_nautical: Option<DateTime<Utc>>,
    pub sunset: Option<DateTime<Utc>>,
    pub sunset_astronomical: Option<DateTime<Utc>>,
    pub sunset_civil: Option<DateTime<Utc>>,
    pub sunset_nautical: Option<DateTime<Utc>>,
    pub temperature_max: f64,
    pub temperature_min: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DailyForecast {
    pub metadata: Metadata,
    pub days: Vec<DayWeatherConditions>,
    pub learn_more_url: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HourWeatherConditions {
    pub cloud_cover: f64,
    pub condition_code: String,
    pub daylight: Option<bool>,
    pub forecast_start: DateTime<Utc>,
    pub humidity: f64,
    pub precipitation_chance: f64,
    pub precipitation_type: PrecipitationType,
    pub pressure: f64,
    pub pressure_trend: Option<PressureTrend>,
    pub snowfall_intensity: Option<f64>,
    pub temperature: f64,
    pub temperature_apparent: f64,
    pub temperature_dew_point: Option<f64>,
    pub uv_index: u8,
    pub visibility: f64,
    pub wind_direction: Option<u16>,
    pub wind_gust: Option<f64>,
    pub wind_speed: f64,
    pub precipitation_amount: Option<f64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HourlyForecast {
    pub metadata: Metadata,
    pub hours: Vec<HourWeatherConditions>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ForecastMinute {
    pub precipitation_chance: f64,
    pub precipitation_intensity: f64,
    pub start_time: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ForecastPeriodSummary {
    pub condition: PrecipitationType,
    pub end_time: Option<DateTime<Utc>>,
    pub precipitation_chance: f64,
    pub precipitation_intensity: f64,
    pub start_time: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NextHourForecast {
    pub metadata: Metadata,
    pub forecast_end: Option<DateTime<Utc>>,
    pub forecast_start: Option<DateTime<Utc>>,
    pub minutes: Vec<ForecastMinute>,
    pub summary: Vec<ForecastPeriodSummary>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Certainty {
    Observed,
    Likely,
    Possible,
    Unlikely,
    Unknown,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ResponseType {
    Shelter,
    Evacuate,
    Prepare,
    Execute,
    Avoid,
    Monitor,
    Assess,
    AllClear,
    None,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Severity {
    Extreme,
    Severe,
    Moderate,
    Minor,
    Unknown,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Urgency {
    Immediate,
    Expected,
    Future,
    Past,
    Unknown,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeatherAlertSummary {
    pub area_id: Option<String>,
    pub area_name: Option<String>,
    pub certainty: Certainty,
    pub country_code: String,
    pub description: String,
    pub details_url: Option<String>,
    pub effective_time: DateTime<Utc>,
    pub event_end_time: Option<DateTime<Utc>>,
    pub event_onset_time: Option<DateTime<Utc>>,
    pub expire_time: DateTime<Utc>,
    pub id: String,
    pub issued_time: DateTime<Utc>,
    pub responses: Vec<ResponseType>,
    pub severity: Severity,
    pub source: String,
    pub urgency: Option<Urgency>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeatherAlertsCollection {
    pub alerts: Vec<WeatherAlertSummary>,
    pub details_url: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub current_weather: Option<CurrentWeather>,
    pub forecast_daily: Option<DailyForecast>,
    pub forecast_hourly: Option<HourlyForecast>,
    pub forecast_next_hour: Option<NextHourForecast>,
    pub weather_alerts: Option<WeatherAlertsCollection>,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DataSet {
    CurrentWeather,
    ForecastDaily,
    ForecastHourly,
    ForecastNextHour,
    WeatherAlerts,
}

impl Display for DataSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataSet::CurrentWeather => write!(f, "currentWeather"),
            DataSet::ForecastDaily => write!(f, "forecastDaily"),
            DataSet::ForecastHourly => write!(f, "forecastHourly"),
            DataSet::ForecastNextHour => write!(f, "forecastNextHour"),
            DataSet::WeatherAlerts => write!(f, "weatherAlerts"),
        }
    }
}
