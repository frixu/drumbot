use super::track::Track;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pattern {
    pub name: String,
    #[serde(alias = "stepCount")]
    #[serde(default)]
    pub step_count: u16,
    #[serde(alias = "beatsPerMinute")]
    #[serde(default)]
    pub beats_per_minute: u16,
    #[serde(default)]
    pub tracks: Vec<Track>
}