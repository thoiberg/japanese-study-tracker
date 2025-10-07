use chrono::{DateTime, Utc};
use chrono_tz::Asia::Tokyo;

pub fn format_date(date: &DateTime<Utc>) -> String {
    format!(
        "{}",
        date.with_timezone(&Tokyo).format("%d/%m/%Y, %I:%M %P %:z")
    )
}

#[cfg(test)]
mod test_super {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_format_date_converts_utc_to_tokyo_time() {
        let utc_time = chrono::Utc
            .with_ymd_and_hms(2025, 10, 7, 9, 39, 00)
            .unwrap();

        let tokyo_date = format_date(&utc_time);

        assert_eq!(tokyo_date, "07/10/2025, 06:39 pm +09:00")
    }
}
