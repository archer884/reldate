//! Library for generating relative datestreams in Rust.
//!
//! ## Abstract (in English)
//!
//! A relative date is like when you've got a meeting on monday, but it's a meeting 
//! that happens on the third day of every month. It isn't possible to add this sort 
//! of thing to a lot of calendars because it's kind of a rare featujre: iOS and 
//! Android both lack this capability out of the box.
//!
//! This library simplifies the task of creating sequences of relative dates so that 
//! programmers can make use of them in their applications.
//!
//! ## Examples
//!
//! <Code samples go here.>

#![feature(std_misc)]
extern crate chrono;
use chrono::{ Local, NaiveDate };

/// Allows iteration of arbitrary date ranges.
///
/// The date range iterator functions as a generator for date ranges bounded on the 
/// seed date; the range will continue to the upper or lower bound of `NaiveDate` 
/// and should be bounded otherwise by a `take()` or `take_while()` iterator adapter.
pub struct DateRangeIterator<F> {
    i: Option<NaiveDate>,
    f: F,
}

impl<F> DateRangeIterator<F>
    where F: FnMut(&NaiveDate) -> Option<NaiveDate>
{
    pub fn new(f: F) -> DateRangeIterator<F> {
        DateRangeIterator::from_date(Local::today().naive_local(), f)
    }

    pub fn from_date(date: NaiveDate, f: F) -> DateRangeIterator<F> {
        DateRangeIterator {
            i: Some(date),
            f: f,
        }
    }
}

impl<F> Iterator for DateRangeIterator<F>
    where F: FnMut(&NaiveDate) -> Option<NaiveDate>
{
    type Item = NaiveDate;

    fn next(&mut self) -> Option<NaiveDate> {
        match self.i {
            Some(i) => {
                let ret = Some(i);
                self.i = (self.f)(&i);
                ret
            },
            None => None
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::{ Datelike, Duration, NaiveDate, Weekday };
    use super::*; // haha super-8

    #[test]
    fn can_detect_dayofweek() {
        let date = NaiveDate::from_ymd(2015, 3, 27);

        assert!(Weekday::Fri == date.weekday());
    }

    #[test]
    fn can_generate_range() {
        let range: Vec<_> = DateRangeIterator::from_date(
                NaiveDate::from_ymd(2015, 3, 27),
                |&d| d.checked_add(Duration::days(1)))
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
}
