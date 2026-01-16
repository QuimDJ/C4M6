use chrono::NaiveDate;
/*use chrono_tz::Etc::UTC;*/

fn main() {
    let d=NaiveDate::from_ymd_opt(2026,01,02);
    println!("{:?}", d);

    let d1="2026-02-29";
    let d1=d1.parse::<NaiveDate>().expect("LA FECHA NO ES CORRECTA!!!.");
    println!("{:?}", d1);


}


