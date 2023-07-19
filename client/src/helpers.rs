use chrono::{DateTime, Local};
use chrono_tz::Europe::Madrid;

pub fn date_convert(date: DateTime<Local>) -> String {
    let madrid_time = date.with_timezone(&Madrid);

    // Adding the offset so that the time zone of the creation of the post is known
    [
        madrid_time.format("%H:%M • %d-%m-%Y").to_string(),
        format!(
            "[UTC{}]",
            DateTime::parse_from_rfc3339(&madrid_time.to_rfc3339())
                .unwrap()
                .offset()
        ),
    ]
    .join(" ")
}

/*
 * DEFINITIVE SOLUTION TO THE PROBLEM OF UTC TIME:
 * https://stackoverflow.com/questions/41158999/getting-the-current-time-in-specified-timezone
 * https://blog.logrocket.com/timezone-handling-in-rust-with-chrono-tz/
 * https://www.iana.org/time-zones
 * https://docs.rs/chrono-tz/0.8.2/chrono_tz/Europe/constant.Madrid.html
 * https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.with_timezone
 */
