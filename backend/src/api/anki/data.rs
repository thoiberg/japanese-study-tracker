use std::fmt;

use chrono::{DateTime, Utc};

#[derive(serde::Serialize)]
pub struct AnkiData {
    active_review_count: u32,
    new_card_count: u32,
    data_updated_at: DateTime<Utc>,
}

// TODO: Figure out better errors
// I want to model specific error states for:
//   - More/Less fields than expected
//   - Fields not being parsable into u32
#[derive(Debug)]
struct MissingHTMLError;

impl std::error::Error for MissingHTMLError {}

impl fmt::Display for MissingHTMLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to find the right HTML")
    }
}

impl AnkiData {
    pub fn new(elements: Vec<String>) -> anyhow::Result<Self> {
        let active_review_count = elements.get(0).ok_or(MissingHTMLError)?.parse::<u32>()?;
        let new_card_count = elements.get(1).ok_or(MissingHTMLError)?.parse::<u32>()?;

        Ok(Self {
            active_review_count,
            new_card_count,
            data_updated_at: Utc::now(),
        })
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_creates_anki_data_with_valid_data() {
        let data = vec!["10".to_owned(), "20".to_owned()];

        let anki_data = AnkiData::new(data).unwrap();

        assert_eq!(anki_data.active_review_count, 10);
        assert_eq!(anki_data.new_card_count, 20);
    }
}
