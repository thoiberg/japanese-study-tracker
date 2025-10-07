use chrono::{DateTime, Utc};
use chrono_tz::Asia::Tokyo;

pub fn format_date(date: &DateTime<Utc>) -> String {
    format!(
        "{}",
        date.with_timezone(&Tokyo).format("%d/%m/%Y, %I:%M %P %:z")
    )
}
