use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    let ndt = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S");
    println!("{:?}", ndt.unwrap());
    let ndt2 = NaiveDate::from_ymd_opt(2015, 9, 5)
        .unwrap()
        .and_hms_opt(23, 56, 4)
        .unwrap();
    println!("{:?}", ndt2);
    assert_eq!(ndt.unwrap(), ndt2);

    let ndt = NaiveDateTime::parse_from_str("5sep2015pm012345.6789", "%d%b%Y%p%I%M%S%.f").unwrap();
    let ndt2 = NaiveDate::from_ymd_opt(2015, 9, 5)
        .unwrap()
        .and_hms_micro_opt(13, 23, 45, 678_900)
        .unwrap();
    println!("{:?}  {:?}", ndt, ndt2);
}
