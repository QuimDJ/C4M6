use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
/*use chrono_tz::Etc::UTC;*/

fn main() {
    let d1 = NaiveDate::parse_from_str("2026/01/01", "%Y/%m/%d").unwrap();
    println!("{d1:?}");
    let t1 = NaiveTime::from_hms_opt(19, 36, 00);
    println!("{:?}", t1);

    let dt = TimeDelta::new(5, 0);
    let five_min = TimeDelta::minutes(5);
    println!("{:?}", five_min);
    println!("{}", five_min.num_days());
    println!("{}", five_min.as_seconds_f64());
    println!("{}", five_min.num_seconds());
    println!("{}", five_min.num_hours());
    println!("{}", five_min.to_string());

    let total = five_min.num_seconds() + five_min.num_seconds();
    println!("{}", total);

    let t1 = NaiveTime::from_hms_opt(20, 0, 00).unwrap();
    let ndt = NaiveDateTime::new(d1, t1);
    // :from_ymd_opt(2020, 01, 01).unwrap();
    let time_add = ndt + five_min;
    println!("{}", time_add);
}
