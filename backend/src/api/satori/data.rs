use chrono::{DateTime, Utc};

#[derive(serde::Serialize)]
pub struct SatoriData {
    data_updated_at: DateTime<Utc>,
    active_review_count: u32,
    new_card_count: u32,
    daily_study_goal_met: bool,
}

impl SatoriData {
    pub fn new(
        current_cards: SatoriCurrentCardsResponse,
        new_cards: SatoriNewCardsResponse,
        stats: SatoriStats,
    ) -> Self {
        Self {
            data_updated_at: current_cards.fetched_at.unwrap_or(Utc::now()),
            active_review_count: current_cards.result,
            new_card_count: new_cards.result,
            daily_study_goal_met: stats.heat_level == SatoriHeatLevel::Four,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SatoriCurrentCardsResponse {
    result: u32,
    success: bool,
    message: Option<String>,
    exception: Option<String>,
    pub fetched_at: Option<DateTime<Utc>>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SatoriNewCardsResponse {
    result: u32,
    success: bool,
    message: Option<String>,
    exception: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SatoriStats {
    pub heat_level: SatoriHeatLevel,
}

#[derive(serde::Deserialize)]
pub struct SatoriHeatData {
    pub date: String,
    pub score: f64,
}

impl SatoriHeatData {
    pub fn heat_level(&self) -> SatoriHeatLevel {
        // HeatMap levels taken from Satori frontend js file
        if self.score > 9.0 {
            SatoriHeatLevel::Four
        } else if self.score > 3.0 {
            SatoriHeatLevel::Three
        } else if self.score > 1.0 {
            SatoriHeatLevel::Two
        } else if self.score > 0.0 {
            SatoriHeatLevel::One
        } else {
            SatoriHeatLevel::Zero
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub enum SatoriHeatLevel {
    Zero,
    One,
    Two,
    Three,
    Four,
}

pub struct InvalidHeatLevel {}

impl TryFrom<&str> for SatoriHeatLevel {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "heat0" => Ok(Self::Zero),
            "heat1" => Ok(Self::One),
            "heat2" => Ok(Self::Two),
            "heat3" => Ok(Self::Three),
            "heat4" => Ok(Self::Four),
            _ => Err(anyhow::anyhow!("uh oh")),
        }
    }
}
