use chrono::{FixedOffset, TimeZone};

const HOUR: std::time::Duration = std::time::Duration::from_secs(3600);
fn main() {
    let offset = FixedOffset::east_opt(8 * HOUR.as_secs() as i32);
    println!("Offset : {:?}", offset);
    let dt = offset.unwrap().with_ymd_and_hms(2018, 12, 08, 12, 34, 56);
    // .with_ymd_and_hms(2028, 08, 28);
    println!("DateTime : {:?}", dt);

    assert_eq!(None, FixedOffset::east_opt(24 * HOUR.as_secs() as i32));

    // let dt = FixedOffset::east_opt(24 * HOUR.as_secs() as i32)
    //     .unwrap()
    //     .with_ymd_and_hms(2018, 12, 08, 12, 34, 56);
    // // .with_ymd_and_hms(2028, 08, 28);
    // println!("{:?}", dt);
}
