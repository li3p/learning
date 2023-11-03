use std::str::FromStr;

use chrono::TimeZone;
use rrule::{Frequency, RRule, RRuleSet, Tz, Unvalidated};

fn main() {
    // let rrule: RRule<Unvalidated> = "FREQ=DAILY;COUNT=40;INTERVAL=3".parse().unwrap();
    // assert_eq!(rrule.get_freq(), Frequency::Daily);
    // assert_eq!(rrule.get_count(), Some(40));
    // assert_eq!(rrule.get_interval(), 3);

    // let start = Tz::UTC.with_ymd_and_hms(2023, 1, 1, 2, 30, 0).unwrap();
    // let rrule_set = RRuleSet::new(start);
    // let rrule = rrule.validate(*rrule_set.get_dt_start()).unwrap();
    // let rrule_set = rrule_set.rrule(rrule);
    // assert_eq!(rrule_set.get_rrule().len(), 1);

    // let rrule_set: RRuleSet = "DTSTART:19960704T120000+08\n\
    // RRULE:FREQ=MONTHLY;COUNT=120\n\
    // RRULE:FREQ=YEARLY;BYMONTH=2,3;COUNT=1\n\
    // RRULE:FREQ=YEARLY;BYMONTH=6,7;COUNT=1"
    //     .parse()
    //     .unwrap();

    // let rrule = rrule_set.get_rrule();
    // assert_eq!(rrule.len(), 3);

    let rrule_set = RRuleSet::from_str(
        "DTSTART;TZID=Asia/Shanghai:20230101T000000\n\
        RRULE:FREQ=MONTHLY;COUNT=120\n\
        RRULE:FREQ=YEARLY;BYMONTH=2,3;COUNT=1\n\
        RRULE:FREQ=YEARLY;BYMONTH=6,7;COUNT=1",
    )
    .unwrap();
    let rrule = rrule_set.get_rrule();
    assert_eq!(rrule.len(), 3);

    let after = Tz::UTC.with_ymd_and_hms(2023, 2, 28, 10, 0, 0).unwrap();

    let result = rrule_set.after(after).all(5);

    for rrule in result.dates {
        println!("{:?}", rrule);
    }
}
