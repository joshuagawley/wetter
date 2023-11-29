// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::tui::border::{Border, Edge, Separator};
use crate::tui::dimension::MIN_CELL_WIDTH;
use crate::weatherkit::{DailyForecast, DayWeatherConditions};
use anyhow::anyhow;
use inflector::Inflector;
use std::cmp;

struct PreparedDailySummary {
    date: String,
    temperature: String,
    condition_code: String,
}

pub struct PreparedDailySummaries {
    summaries: Vec<PreparedDailySummary>,
    width: usize,
}

impl DayWeatherConditions {
    fn prepare(&self) -> PreparedDailySummary {
        let date = format!("{}", self.forecast_start.format("%a, %h %d"));
        let temperature = format!(
            "{:.1}ºC/{:.1}ºC",
            self.temperature_max, self.temperature_min
        );

        PreparedDailySummary {
            date,
            temperature,
            condition_code: self.condition_code.to_title_case(),
        }
    }
}

impl DailyForecast {
    pub fn prepare(self) -> anyhow::Result<PreparedDailySummaries> {
        let summaries = self.days.iter().map(|x| x.prepare()).collect::<Vec<_>>();
        let width = summaries
            .iter()
            .map(|x| x.condition_code.len() + x.temperature.len() + x.condition_code.len())
            .max()
            .ok_or_else(|| {
                anyhow!("Internal error: could not get maximum of prepared daily summaries.")
            })?
            + 10;

        Ok(PreparedDailySummaries { summaries, width })
    }
}

impl PreparedDailySummary {
    fn render(&self, term_width: usize, cell_width: usize) -> String {
        let width = term_width - cmp::max(11, self.date.len()) - self.temperature.len() - 5
            + cell_width
            - MIN_CELL_WIDTH;

        format!(
            "{: <cell_width$}{} {: >width$}",
            self.date, self.temperature, self.condition_code,
        )
    }
}

impl PreparedDailySummaries {
    pub fn render(self) {
        let term_width = self.width;
        let cell_width = MIN_CELL_WIDTH;

        // Border top
        println!("{}", Edge::Top.fmt(term_width));

        let mut iter = self.summaries.iter().peekable();

        while let Some(summary) = iter.next() {
            println!(
                "{} {: <width$} {}",
                Border::Left.fmt(),
                summary.render(term_width, cell_width),
                Border::Right.fmt(),
                width = term_width - 2
            );

            if iter.peek().is_some() {
                // Separator
                println!("{}", Separator::Dashed.fmt(term_width));
            }
        }

        // Border bottom
        println!("{}", Edge::Bottom.fmt(term_width));
    }
}
