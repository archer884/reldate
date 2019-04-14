//! Library for generating relative datestreams in Rust.
//!
//! # Abstract (in English)
//!
//! A relative date is like when you've got a meeting on monday, but it's a meeting
//! that happens on the third day of every month. It isn't possible to add this sort
//! of thing to a lot of calendars because it's kind of a rare feature: iOS and
//! Android both lack this capability out of the box.
//!
//! This library simplifies the task of creating sequences of relative dates so that
//! programmers can make use of them in their applications.
//!
//! ## Examples
//!
//! <Code samples go here.>

//#![feature(box_syntax, std_misc)]
extern crate chrono;
use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};

/// Allows iteration of arbitrary date ranges.
///
/// The date range iterator functions as a generator for date ranges bounded on the
/// seed date; the range will continue to the upper or lower bound of `NaiveDate`
/// and should be bounded otherwise by a `take()` or `take_while()` iterator adapter.
pub struct DateRangeIterator<F> {
    date: Option<NaiveDate>,
    function: F,
}

impl<F> DateRangeIterator<F>
where
    F: FnMut(&NaiveDate) -> Option<NaiveDate>,
{
    pub fn new(f: F) -> DateRangeIterator<F> {
        DateRangeIterator::from_date(Local::today().naive_local(), f)
    }

    pub fn from_date(date: NaiveDate, function: F) -> DateRangeIterator<F> {
        DateRangeIterator {
            date: Some(date),
            function,
        }
    }
}

impl<F> Iterator for DateRangeIterator<F>
where
    F: FnMut(&NaiveDate) -> Option<NaiveDate>,
{
    type Item = NaiveDate;

    fn next(&mut self) -> Option<NaiveDate> {
        match self.date {
            Some(date) => {
                let ret = Some(date);
                self.date = (self.function)(&date);
                ret
            }
            None => None,
        }
    }
}

pub fn weekday_iterator(date: NaiveDate, day: Weekday) -> Box<Iterator<Item = NaiveDate>> {
    Box::new(DateRangeIterator::from_date(date, move |d| weekday_incrementor(*d, day)).skip(1))
}

fn weekday_incrementor(date: NaiveDate, day: Weekday) -> Option<NaiveDate> {
    let mut date = date;
    loop {
        date = match date.checked_add_signed(Duration::days(1)) {
            Some(date) if date.weekday() == day => return Some(date),
            Some(date) => date,
            None => return None,
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Datelike, Duration, NaiveDate, Weekday}; // haha super-8

    #[test]
    fn can_detect_dayofweek() {
        let date = NaiveDate::from_ymd(2015, 3, 27);

        assert!(Weekday::Fri == date.weekday());
    }

    #[test]
    fn can_generate_range() {
        let range: Vec<_> = DateRangeIterator::from_date(NaiveDate::from_ymd(2015, 3, 27), |&d| {
            d.checked_add_signed(Duration::days(1))
        })
        .take(5)
        .collect();
        println!("{:?}", range); // output not printed on success?

        let test_range = [
            NaiveDate::from_ymd(2015, 3, 27),
            NaiveDate::from_ymd(2015, 3, 28),
            NaiveDate::from_ymd(2015, 3, 29),
            NaiveDate::from_ymd(2015, 3, 30),
            NaiveDate::from_ymd(2015, 3, 31),
        ];

        assert!(range == test_range);
    }

    #[test]
    fn can_generate_weekly_range() {
        let range: Vec<_> = super::weekday_iterator(NaiveDate::from_ymd(2015, 4, 19), Weekday::Tue)
            .take(5)
            .collect();

        let test_range = [
            NaiveDate::from_ymd(2015, 4, 21),
            NaiveDate::from_ymd(2015, 4, 28),
            NaiveDate::from_ymd(2015, 5, 5),
            NaiveDate::from_ymd(2015, 5, 12),
            NaiveDate::from_ymd(2015, 5, 19),
        ];

        for date in &range {
            println!("{}", date);
        }

        assert!(range == test_range);
    }
}
