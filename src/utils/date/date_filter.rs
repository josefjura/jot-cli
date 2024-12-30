use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::date_value::DateValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DateFilter {
    SpecificDate(DateValue),
    Range(DateValue, DateValue),
}

impl FromStr for DateFilter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("..").collect();
        match parts.len() {
            1 => Ok(Self::SpecificDate(parts[0].parse()?)),
            2 => Ok(Self::Range(parts[0].parse()?, parts[1].parse()?)),
            _ => Err(anyhow::anyhow!("Invalid date filter")),
        }
    }
}

#[cfg(test)]
mod date_value_test {
    use chrono::NaiveDate;

    use crate::utils::date::date_filter::DateFilter;

    use crate::utils::date::date_value::DateValue;

    #[test]
    fn test_specific_ever() {
        let value = "".parse::<DateFilter>().unwrap();

        assert_eq!(value, DateFilter::SpecificDate(DateValue::Ever));
    }

    #[test]
    fn test_specific_today() {
        let value = "today".parse::<DateFilter>().unwrap();

        assert_eq!(value, DateFilter::SpecificDate(DateValue::Today));
    }

    #[test]
    fn test_specific_date() {
        let value = "2024-01-01".parse::<DateFilter>().unwrap();

        assert_eq!(
            value,
            DateFilter::SpecificDate(DateValue::Date(
                NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
            ))
        );
    }

    #[test]
    fn test_range_from_today() {
        let value = "today..".parse::<DateFilter>().unwrap();

        assert_eq!(value, DateFilter::Range(DateValue::Today, DateValue::Ever));
    }

    #[test]
    fn test_range_until_today() {
        let value = "..today".parse::<DateFilter>().unwrap();

        assert_eq!(value, DateFilter::Range(DateValue::Ever, DateValue::Today));
    }

    #[test]
    fn test_range_two_dates() {
        let value = "2024-01-01..2025-01-01".parse::<DateFilter>().unwrap();

        assert_eq!(
            value,
            DateFilter::Range(
                DateValue::Date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
                DateValue::Date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
            )
        );
    }

    #[test]
    fn test_range_with_spaces() {
        let value = "week ago..in a week".parse::<DateFilter>().unwrap();

        assert_eq!(
            value,
            DateFilter::Range(DateValue::WeekAgo, DateValue::InAWeek)
        );
    }
}
