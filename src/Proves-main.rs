use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use std::ops::{Add, Sub};

/*use chrono_tz::Etc::UTC;*/

fn main() {
    let d1 = NaiveDate::from_ymd_opt(2026, 1, 15);
    let t1 = NaiveTime::from_hms_opt(21, 0, 0);
    println!(" d1: {:?}\n t1: {:?}", d1, t1);
    let four_thirty_am = NaiveTime::from_hms_opt(4, 30, 0);
    let four_thirty_pm = NaiveTime::from_hms_opt(16, 30, 0);
    println!("{:?}   {:?}", four_thirty_am, four_thirty_pm);

    // Moonlight Landing
    let moon_light_landing = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(1969, 7, 20).unwrap(),
        NaiveTime::from_hms_opt(8, 17, 0).unwrap(),
    );
    println!("{:?}", moon_light_landing);
}
