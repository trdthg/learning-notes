pub fn get_interval() {
    use std::time;

    let start = time::Instant::now();
    std::thread::sleep(time::Duration::from_millis(500));
    println!("test takes {:?}", start.elapsed());
}

pub fn get_sometimes_later() {
    use chrono::{DateTime, Duration, Utc};
    let now = Utc::now();

    fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
        date_time.checked_add_signed(Duration::days(1))
    }
    let almost_three_weeks_later = now
        .checked_add_signed(Duration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
        .and_then(day_earlier);
    match almost_three_weeks_later {
        Some(x) => println!("now: {}\nthree weeks later: {}", now, x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }
    println!("{}", chrono::MAX_DATE);
}

pub fn local_to_utc() {
    use chrono::{DateTime, FixedOffset, Local, Utc};
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from(local_time);
    println!("{}   {}", local_time, utc_time);
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    println!("{}   {}", local_time, utc_time);

    let china_timezone = FixedOffset::east(8 * 3600);
    let rio_timezone = FixedOffset::west(2 * 3600);
    println!("utc_time: {}", utc_time.with_timezone(&china_timezone));
    println!("rio_time: {}", utc_time.with_timezone(&rio_timezone));
}

pub fn get_timeinfo() {
    use chrono::{Datelike, Timelike, Utc};
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    println!(
        "The current UTC time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    println!(
        "And there have been {} seconds since midnight",
        now.num_seconds_from_midnight()
    );
    let (is_common_era, year) = now.year_ce();
    println!(
        "The current UTC date is {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" }
    );
    println!(
        "And the Common Era began {} days ago",
        now.num_days_from_ce()
    );
}

pub fn datetime_to_timestamp() {
    use chrono::{NaiveDate, NaiveDateTime};
    let date_time: NaiveDateTime = NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44);
    println!(
        "
        Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
        date_time,
        date_time.timestamp()
    );
    let date_time: NaiveDateTime = NaiveDateTime::from_timestamp(1_000_000_000, 0);
    println!(
        "
        Date after {} seconds since 1970-01-01 00:00:00 was {} .",
        date_time.timestamp(),
        date_time
    );
}

pub fn format_display() {
    use chrono::Utc;

    let now = Utc::now();
    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!(
        "UTC now in a custom format is: {}",
        now.format("%a %b %e %T %Y")
    );
}

use chrono::format::ParseError;
pub fn parse_time_string_to_struct() -> Result<(), ParseError> {
    use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

    let rfc2822 = DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("{}", rfc2822);

    let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("{}", rfc3339);

    let custom = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z")?;
    println!("{}", custom);

    let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("{}", time_only);

    let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("{}", date_only);

    let no_timezone = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;
    println!("{}", no_timezone);

    Ok(())
}

#[test]
pub fn test() {
    // get_interval();
    get_sometimes_later();
    // local_to_utc();
    // datetime_to_timestamp();
    // format_display();
    // parse_time_string_to_struct();
}
