use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Track {
    pub instrument: String,
    pub steps: Vec<bool>
}