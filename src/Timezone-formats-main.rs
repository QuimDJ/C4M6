use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use chrono_tz::{America::Santiago, Europe::Madrid};
use std::ops::{Add, Sub};

/*use chrono_tz::Etc::UTC;*/

fn main() {
    let system_time = Local::now();
    let utc_time = Utc::now();
    let bcn_time = Local::now().with_timezone(&Madrid);
    let santiago_time = Utc::now().with_timezone(&Santiago);
    let spain_time = Utc::now().with_timezone(&Madrid);
    println!("{system_time:?}    {utc_time:?}");
    println!("{bcn_time:?}");
    println!("{santiago_time:?}");

    println!("Santiago time zone: {:?}", santiago_time.timezone());
    println!("Spain time: {:?}", spain_time);
    println!("Spain time: {:?}", spain_time.timestamp());
    println!("Spain time: {}", spain_time.format("%d/%m/%Y - %H:%M:%S"));
    let xile = Local::now().with_timezone(&Santiago);
    println!(
        "Santiago de Xile: {}",
        xile.format("%A, %d-%b-%Y %I:%M %p Timezone: %Z")
    );
}
