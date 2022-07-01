use super::error::WesternDateError;
use chrono::naive::NaiveDate;
use chrono::prelude::*;
use chrono::Duration;

/// calculate the date of Advent Sunday: the fourth Sunday before Christmas Day,
/// and the beginning of the Western liturgical calendar
pub fn date(year: i32) -> Result<NaiveDate, WesternDateError> {
    if year < 1583 {
        return Err(WesternDateError::InvalidPrior1583);
    }
    let christmas = NaiveDate::from_ymd(year, 12, 25);
    // the number of days we need to seek back for the Sunday prior to Christmas Day
    // (if Christmas Day is on a Sunday, this is 7)
    let sunday_offset = christmas.weekday() as i64 + 1;
    println!("{}", sunday_offset);
    let advent_sunday = christmas + Duration::days(-21 - sunday_offset);

    return Ok(advent_sunday);
}

#[cfg(test)]
mod tests {
    #[test]
    fn advent_valid_dates() {
        use super::date;
        use chrono::naive::NaiveDate;
        fn td(y: i32, m: u32, d: u32) {
            assert_eq!(date(y), Ok(NaiveDate::from_ymd(y, m, d)));
        }
        td(2020, 11, 29);
        td(2022, 11, 27);
        td(2023, 12, 3);
    }

    #[test]
    fn fail_before_gregory() {
        use super::super::error::WesternDateError;
        use super::date;
        assert_eq!(date(1582), Err(WesternDateError::InvalidPrior1583));
    }
}
