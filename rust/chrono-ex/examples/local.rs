use chrono::{Local, NaiveDate, NaiveDateTime, TimeZone};

fn main() {
    let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 7, 8)
        .unwrap()
        .and_hms_opt(9, 10, 11)
        .unwrap();
    println!("{:?}", dt);

    let dt = Local.from_utc_datetime(&dt);

    println!("{:?}", dt);
}
