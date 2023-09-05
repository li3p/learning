use chrono::*;

fn main() {
    let utc = Utc::now();
    println!("UTC now: {}", utc);
}

#[test]
fn datetime() {
    // 1. Date
    // Date 只有年月日, 例如 2021-1-4.
    // 即： 2021-1-4 => Date = NaiveDate

    // 2. Time
    // Time 只有 时间，例如: 19:28:33.
    // 即: 19:28:33 => Date = NaiveDate

    // 3. Date 和 Time
    // DateTime 包含日期和时间.
    // 即：2021-1-4 19:28:33 => Date + Time = NaiveDateTime

    // 4. DateTime
    // 真正的 DateTime 里面包含了日期，时间以及时区。
    // 即：2021-1-4 19:28:33+08:00 => Date + Time + TimeZone = DateTime

    // 获取当前时间
    let dt: DateTime<Local> = Local::now();
    println!("dt={dt:#?}");

    println!("dt.date_naive()={:#?}", dt.date_naive());

    // 获取时间戳（秒）
    let timestamp = dt.timestamp();
    println!("dt.timestamp()={:#?}", timestamp);

    // 时间戳转 DateTime
    let new_utc_dt = Utc.timestamp_opt(timestamp + 30, 0).unwrap();
    // let new_dt = NaiveDateTime::from_timestamp_millis((timestamp + 30) * 1000).unwrap();
    println!("new_utc_dt={:#?}", new_utc_dt);

    let new_local_dt: DateTime<Local> = Local.timestamp_opt(timestamp + 30, 0).unwrap();
    println!("new_local_dt={:#?}", new_local_dt);

    // 东八区TZ
    let tz_offset = FixedOffset::east_opt(8 * 3600).unwrap();
    println!("tz_offset={:#?}", tz_offset);

    // 由DateTime<Utc> 转 带时区的 DateTime
    let new_utc_dt_with_tz = tz_offset.from_utc_datetime(&new_utc_dt.naive_utc());
    println!("new_utc_dt_with_tz={:#?}", new_utc_dt_with_tz);

    // DateTime<Local> == DateTime<FixedOffset> FixedOffset需要是东八区
    assert_eq!(new_utc_dt_with_tz, new_local_dt);
}
