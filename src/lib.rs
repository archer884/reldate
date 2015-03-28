//! Library for generating relative date streams in Rust.
//!
//! ## Translation
//!
//! A relative date is like when you've got a meeting on Monday, but it's a meeting that happens
//! the third Monday of every month. You can add these kinds of events to some calendars, but it's
//! actually a pretty rare sort of function: standard calendars like the one provided in iOS or
//! the one provided in Android don't include this.
//!
//! This library is designed to permit the programmer to generate such date sequences without a
//! lot of effort, so that there's no excuse for the next big calendar app not to include it. ;)
//!
//! ## Examples
//!
//! <Code samples go here.>

#![feature(std_misc)]
extern crate chrono;

use chrono::{ Duration, Local, NaiveDate, Weekday };

/// Allows iteration of arbitrary date ranges.
///
/// The date range iterator functions as a generator for date ranges bounded only on the "start"
/// side; they continue in whatever direction forever (or, rather, until the date range for the
/// underlying NaiveDate type is exhausted).
pub struct DateRangeIterator<F>
{
    i: NaiveDate,   // iterative date value
    f: F            // incrementor function
}

impl<F> DateRangeIterator<F>
    where F: FnMut(&NaiveDate) -> NaiveDate
{
    pub fn new(incrementor: F) -> DateRangeIterator<F> {
        DateRangeIterator { i: Local::today().naive_local(), f: incrementor }
    }

    pub fn from_date(date: NaiveDate, incrementor: F) -> DateRangeIterator<F> {
        DateRangeIterator { i: date, f: incrementor }
    }
}

impl<F> Iterator for DateRangeIterator<F>
    where F: FnMut(&NaiveDate) -> NaiveDate
{
    type Item = NaiveDate;

    fn next(&mut self) -> Option<NaiveDate> {
        let ret = Some(self.i);
        self.i = (self.f)(&self.i);
        ret
    }
}

#[cfg(test)]
mod test {
    use chrono::{ Datelike, Duration, NaiveDate, Weekday };
    use super::*;

    #[test]
    fn can_generate_range(){
        let range: Vec<_> = DateRangeIterator::from_date(
                NaiveDate::from_ymd(2000, 12, 25),
                |&d| d + Duration::days(1))
            .take(5)
            .collect();

        let test_range = [
            NaiveDate::from_ymd(2000, 12, 25),
            NaiveDate::from_ymd(2000, 12, 26),
            NaiveDate::from_ymd(2000, 12, 27),
            NaiveDate::from_ymd(2000, 12, 28),
            NaiveDate::from_ymd(2000, 12, 29),
        ];

        assert!(range == test_range);
    }

    #[test]
    fn can_detect_day() {
        let date = NaiveDate::from_ymd(2015, 3, 27);

        assert!(Weekday::Fri == date.weekday());
    }
}
