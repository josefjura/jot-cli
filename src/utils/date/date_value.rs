use std::{fmt::Display, str::FromStr};

use chrono::{Days, Months, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DateValue {
    Date(NaiveDate),
    Today,
    Tomorrow,
    Yesterday,
    WeekAgo,
    InAWeek,
    MonthAgo,
    InAMonth,
    YearAgo,
    InAYear,
    Ever,
}

impl FromStr for DateValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "today" => Ok(Self::Today),
            "tomorrow" => Ok(Self::Tomorrow),
            "yesterday" => Ok(Self::Yesterday),
            "week ago" => Ok(Self::WeekAgo),
            "in a week" => Ok(Self::InAWeek),
            "month ago" => Ok(Self::MonthAgo),
            "in a month" => Ok(Self::InAMonth),
            "year ago" => Ok(Self::YearAgo),
            "in a year" => Ok(Self::InAYear),
            "" => Ok(Self::Ever),
            _ => Ok(Self::Date(NaiveDate::parse_from_str(s, "%Y-%m-%d")?)),
        }
    }
}

impl DateValue {
    pub fn to_date(&self, today: NaiveDate) -> Option<NaiveDate> {
        match self {
            Self::Today => Some(today),
            Self::Tomorrow => today.succ_opt(),
            Self::Yesterday => today.pred_opt(),
            Self::WeekAgo => today.checked_sub_days(Days::new(7)),
            Self::InAWeek => today.checked_add_days(Days::new(7)),
            Self::MonthAgo => today.checked_sub_months(Months::new(1)),
            Self::InAMonth => today.checked_add_months(Months::new(1)),
            Self::YearAgo => today.checked_sub_months(Months::new(12)),
            Self::InAYear => today.checked_add_months(Months::new(12)),
            Self::Ever => None,
            Self::Date(date) => Some(*date),
        }
    }
}

impl Display for DateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Today => f.write_str("today")?,
            Self::Tomorrow => f.write_str("tomorrow")?,
            Self::Yesterday => f.write_str("yesterday")?,
            Self::WeekAgo => f.write_str("week ago")?,
            Self::InAWeek => f.write_str("in a week")?,
            Self::MonthAgo => f.write_str("month ago")?,
            Self::InAMonth => f.write_str("in a month")?,
            Self::YearAgo => f.write_str("year ago")?,
            Self::InAYear => f.write_str("in a year")?,
            Self::Ever => f.write_str("")?,
            Self::Date(date) => f.write_str(&date.format("%Y-%m-%d").to_string())?,
        };

        Ok(())
    }
}
