use chrono::{DateTime, Utc};

use super::proto_definitions::DeckInfo;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AnkiData {
    active_review_count: u32,
    total_active_review_count: u32,
    new_card_count: u32,
    total_new_card_count: u32,
    data_updated_at: DateTime<Utc>,
    daily_study_goal_met: bool,
}

impl From<DeckInfo> for AnkiData {
    fn from(deck: DeckInfo) -> Self {
        Self {
            active_review_count: deck.review_card_count() + deck.learn_count(),
            total_active_review_count: deck.uncapped_review_card_count(),
            new_card_count: deck.new_card_count(),
            total_new_card_count: deck.uncapped_new_card_count(),
            data_updated_at: Utc::now(),
            daily_study_goal_met: deck.review_card_count() == 0,
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn create_deck_info(
        review_card_count: Option<u32>,
        learn_count: Option<u32>,
        new_card_count: Option<u32>,
    ) -> DeckInfo {
        DeckInfo {
            deck_id: 1,
            deck_name: String::from("Japanese"),
            level: 3,
            review_card_count,
            learn_count,
            new_card_count,
            uncapped_new_card_count: None,
            uncapped_review_card_count: None,
            total_card_count: 10,
        }
    }

    #[test]
    fn test_create_anki_data_with_no_reviews_or_new_cards() {
        let deck_info = create_deck_info(None, None, None);

        let anki_data: AnkiData = AnkiData::from(deck_info);

        assert_eq!(anki_data.active_review_count, 0);
        assert_eq!(anki_data.new_card_count, 0);
    }

    #[test]
    fn test_create_anki_data_with_reviews_and_new_cards() {
        let deck_info = create_deck_info(Some(10), None, Some(5));

        let anki_data: AnkiData = AnkiData::from(deck_info);

        assert_eq!(anki_data.active_review_count, 10);
        assert_eq!(anki_data.new_card_count, 5);
    }

    #[test]
    fn test_create_anki_data_with_learning_and_review_cards() {
        let deck_info = create_deck_info(Some(10), Some(10), None);

        let anki_data = AnkiData::from(deck_info);

        assert_eq!(anki_data.active_review_count, 20);
        assert_eq!(anki_data.new_card_count, 0);
    }
}
