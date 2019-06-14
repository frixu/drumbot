use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Track {
    #[doc = "Name of the instrument that plays this track."]
    pub instrument: String,
    #[doc = "Values that determine behaviour of the instrument in this tick."]
    #[doc = "They can be either 0 for mute or 1 for playback."]
    pub steps: Vec<u8>
}