// use chrono::offset::LocalResult;
use chrono::{prelude::*, LocalResult};

fn main() {
    let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
                                                                   // July 8 is 188th day of the year 2014 (`o` for "ordinal")
    assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
    // July 8 is Tuesday in ISO week 28 of the year 2014.
    assert_eq!(
        dt,
        Utc.isoywd(2014, 28, Weekday::Tue)
            .and_hms_opt(9, 10, 11)
            .unwrap()
    );

    let dt = NaiveDate::from_ymd_opt(2014, 7, 8)
        .unwrap()
        .and_hms_milli_opt(9, 10, 11, 12)
        .unwrap()
        .and_local_timezone(Utc)
        .unwrap(); // `2014-07-08T09:10:11.012Z`
    assert_eq!(
        dt,
        NaiveDate::from_ymd_opt(2014, 7, 8)
            .unwrap()
            .and_hms_micro_opt(9, 10, 11, 12_000)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
    );
    assert_eq!(
        dt,
        NaiveDate::from_ymd_opt(2014, 7, 8)
            .unwrap()
            .and_hms_nano_opt(9, 10, 11, 12_000_000)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
    );

    // dynamic verification
    assert_eq!(
        Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
        LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap())
    );
    assert_eq!(
        Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33),
        LocalResult::None
    );
    assert_eq!(
        Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33),
        LocalResult::None
    );

    // other time zone objects can be used to construct a local datetime.
    // obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
    let local_dt = Local
        .from_local_datetime(
            &NaiveDate::from_ymd_opt(2014, 7, 8)
                .unwrap()
                .and_hms_milli_opt(9, 10, 11, 12)
                .unwrap(),
        )
        .unwrap();
    let fixed_dt = FixedOffset::east_opt(9 * 3600)
        .unwrap()
        .from_local_datetime(
            &NaiveDate::from_ymd_opt(2014, 7, 8)
                .unwrap()
                .and_hms_milli_opt(18, 10, 11, 12)
                .unwrap(),
        )
        .unwrap();
    assert_eq!(dt, fixed_dt);
}
