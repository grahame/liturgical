use chrono::naive::NaiveDate;

#[derive(Debug, Eq, PartialEq)]
pub enum EasterError {
    NoYearZero,
    InvalidPrior1583,
}

/// calculate Easter according to the Gregorian calendar of the Western church,
/// for the given year. Algorithm comes from Knuth, The Art of Computer Programming,
/// Third Edition, p160
pub fn date(year: i32) -> Result<NaiveDate, EasterError> {
    if year == 0 {
        return Err(EasterError::NoYearZero);
    }
    if year < 1583 {
        return Err(EasterError::InvalidPrior1583);
    }
    // the Gregorian calendar came in
    let golden_number = year.rem_euclid(19) + 1;
    let century = (year / 100) + 1;
    let dropped_leap_years = (3 * century) / 4 - 12;
    let moon_correction = ((8 * century + 5) / 25) - 5;
    let sunday = (5 * year) / 4 - dropped_leap_years - 10;
    let mut epact = (11 * golden_number + 20 + moon_correction - dropped_leap_years).rem_euclid(30);
    if ((epact == 25) && (golden_number > 11)) || (epact == 24) {
        epact += 1;
    }
    let mut calendar_moon = 44 - epact;
    if calendar_moon < 21 {
        calendar_moon += 30;
    }
    let advance_sunday = calendar_moon + 7 - (sunday + calendar_moon).rem_euclid(7);
    if advance_sunday > 31 {
        // our answer is in April
        return Ok(NaiveDate::from_ymd(year, 4, (advance_sunday - 31) as u32));
    } else {
        // oura nswer is in March
        return Ok(NaiveDate::from_ymd(year, 3, advance_sunday as u32));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn easter_valid_dates() {
        use super::date;
        use chrono::naive::NaiveDate;
        fn td(y: i32, m: u32, d: u32) {
            assert_eq!(date(y), Ok(NaiveDate::from_ymd(y, m, d)));
        }
        td(1583, 4, 10);
        td(1981, 4, 19);
        td(2013, 3, 31);
        td(2016, 3, 27);
        td(2022, 4, 17);
        td(2023, 4, 9);
        // example from Knuth, will fail if we had simply used '%' instead of rem_euclid()
        td(14250, 4, 14);
    }

    #[test]
    fn fail_before_gregory() {
        use super::date;
        use super::EasterError;
        assert_eq!(date(1582), Err(EasterError::InvalidPrior1583));
    }
}
