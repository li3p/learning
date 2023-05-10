use chrono::{DateTime, FixedOffset, Utc};

fn main() {
    let utc_timestamp = Utc::now();
    let fixed_offset_timestamp =
        utc_timestamp.with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
    println!("Utc::now(): {:?}", utc_timestamp); // 输出：2023-05-09T06:55:02.969853Z
    println!("with_timezone(east+8): {:?}", fixed_offset_timestamp); // 输出：2023-05-09T06:55:02.969853Z

    println!("NaiveDateTime: {:?}", chrono::Local::now().naive_local());

    // 获取本地时间
    let local_time = chrono::Local::now();
    println!("Local time: {:?}", local_time);
    println!("Local time Offset: {:?}", local_time.offset());

    // 转换为 UTC 时间
    let utc_time: DateTime<Utc> = local_time.with_timezone(&Utc);
    println!("UTC time: {:?}", utc_time);

    // 转换为带有时区偏移量的时间
    let fixed_offset_time: DateTime<FixedOffset> =
        local_time.with_timezone(&&FixedOffset::east_opt(8 * 3600).unwrap());
    println!("FixedOffset time: {:?}", fixed_offset_time);
}
