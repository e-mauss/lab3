#![feature(iter_advance_by)]

use std::ops::Range;
use chrono::{Datelike, NaiveDate, TimeZone, Utc, Weekday};
use lab3::CartesianIteratorExt;

pub fn month_iter(start_year: i32, start_month: u32) -> impl Iterator<Item=String> {
    (1..13).cartesian(start_year..).skip((start_month - 1) as usize)
        .map(|(month, year)| {
            format!("{}{}",generate_calendar_head(month, year), get_days_from_month(year, month)
                .map(|day| {
                    generate_day_string(month, year, day)
                }).collect::<String>())

        })
}

fn generate_day_string(month: u32, year: i32, day: u32) -> String {
    match get_weekday_from_day(year, month, day) {
        Weekday::Sun => { format!("{:>3}\n", day) }
        _ => { format!("{:>3}", day) }
    }
}

fn generate_calendar_head(month: u32, year: i32) -> String {
    match get_weekday_from_day(year, month, 1) {
        Weekday::Mon => { " Mo Tu We Th Fr Sa So\n" }
        Weekday::Tue => { " Mo Tu We Th Fr Sa So\n   " }
        Weekday::Wed => { " Mo Tu We Th Fr Sa So\n      " }
        Weekday::Thu => { " Mo Tu We Th Fr Sa So\n         " }
        Weekday::Fri => { " Mo Tu We Th Fr Sa So\n            " }
        Weekday::Sat => { " Mo Tu We Th Fr Sa So\n               " }
        Weekday::Sun => { " Mo Tu We Th Fr Sa So\n                  " }
    }.to_owned()
}

fn get_weekday_from_day(start_year: i32, start_month: u32, day: u32) -> Weekday {
    Utc.ymd(start_year, start_month, day).weekday()
}

fn get_days_from_month(year: i32, month: u32) -> Range<u32> {
    let num_days = NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    ).signed_duration_since(NaiveDate::from_ymd(year, month, 1)).num_days() as u32;

    1..num_days + 1
}

fn main() {
    let mut iter = month_iter(2020, 24);
    print!("{}\n", iter.next().unwrap());
    print!("{}\n", iter.next().unwrap());
    print!("{}\n", iter.next().unwrap());
    print!("{}\n", iter.next().unwrap());
    print!("{}\n", iter.next().unwrap());
}