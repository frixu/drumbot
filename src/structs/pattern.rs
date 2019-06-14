use super::track::Track;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pattern {
    #[doc = "Name of the pattern."]
    pub name: String,
    #[serde(alias = "stepCount")]
    #[serde(default)]
    #[doc = "How long is this pattern?"]
    pub step_count: u16,
    #[serde(alias = "beatsPerMinute")]
    #[serde(default)]
    #[doc = "Tempo of this loop."]
    pub beats_per_minute: u16,
    #[serde(default)]
    #[doc = "Instruments used in this loop."]
    pub tracks: Vec<Track>
}