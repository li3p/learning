use chrono::{Local, TimeZone, Utc};
use rrule::{Frequency, RRule, RRuleSet, Tz, Unvalidated};

fn main() {
    let rule1: RRule<Unvalidated> = "RRULE:FREQ=YEARLY;BYMONTH=2,3".parse().unwrap();
    assert_eq!(Frequency::Yearly, rule1.get_freq());
    assert_eq!(1, rule1.get_interval());
    assert_eq!(None, rule1.get_count());
    let tz = Tz::Asia__Shanghai;

    // let rule2 = rule1.until(tz.with_ymd_and_hms(2028, 12, 31, 00, 00, 00).unwrap());
    // assert_eq!(3, rule2.get_count().unwrap());

    let rrule_set: RRuleSet = "DTSTART:20120201T023000Z\n\
    RRULE:FREQ=MONTHLY;UNTIL=20121201T023000Z\n\
    RDATE:20120701T123000Z,20120702T223000Z\n\
    EXDATE:20120601T023000Z"
        .parse()
        .unwrap();

    assert_eq!(
        *rrule_set.get_dt_start(),
        Tz::UTC.with_ymd_and_hms(2012, 2, 1, 2, 30, 0).unwrap()
    );

    let rules = rrule_set.get_rrule();
    assert_eq!(1, rules.len());

    // let rule3 = &rules[0];
    // assert_eq!(8, rule3.get_count().unwrap());

    let all = rrule_set.into_iter();
    for item in all {
        println!("item={:#?}", item);
    }
}

#[test]

fn test_birds_schedule() {
    let rrule_set: RRuleSet = "DTSTART:20230101T000000Z\n\
    RRULE:FREQ=MONTHLY;COUNT=24\n\
    RRULE:FREQ=YEARLY;BYMONTH=2,3;COUNT=1\n\
    RRULE:FREQ=YEARLY;BYMONTH=6,7;COUNT=1"
        .parse()
        .unwrap();

    assert_eq!(
        *rrule_set.get_dt_start(),
        Tz::UTC.with_ymd_and_hms(2023, 1, 1, 0, 00, 0).unwrap()
    );

    let rules = rrule_set.get_rrule();
    assert_eq!(3, rules.len());

    // let rule3 = &rules[0];
    // assert_eq!(10, rule3.get_count().unwrap());

    // for line in rrule_set.into_iter() {
    //     println!("line={:#?}", line);
    // }

    // Between two dates
    let after = Tz::UTC.with_ymd_and_hms(2023, 12, 1, 10, 0, 0).unwrap();
    let before = Tz::UTC.with_ymd_and_hms(2024, 4, 1, 9, 0, 0).unwrap();

    let rrule_set = rrule_set.after(after).before(before);
    for line in rrule_set.into_iter() {
        println!("line={:#?}", line);
    }
}

#[test]
fn test_before_after() {
    use chrono::{DateTime, TimeZone};
    use rrule::{RRuleSet, Tz};

    let rrule: RRuleSet = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=10"
        .parse()
        .unwrap();

    // Between two dates
    let after = Tz::UTC.with_ymd_and_hms(2012, 2, 1, 10, 0, 0).unwrap();
    let before = Tz::UTC.with_ymd_and_hms(2012, 2, 3, 10, 0, 0).unwrap();

    let rrule = rrule.after(after).before(before);
    let result = rrule.all(100);

    assert_eq!(
        vec![
            DateTime::parse_from_rfc3339("2012-02-02T09:30:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2012-02-03T09:30:00+00:00").unwrap(),
        ],
        result.dates
    );
}


#[test]
fn compose_rrule_by_code() {
    
}