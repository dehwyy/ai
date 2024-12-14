use serde::Serialize;

#[derive(Default)]
pub enum SampleRate {
    Low,    // 8000
    Medium, // 24000
    #[default]
    High, // 48000
}

impl TryFrom<String> for SampleRate {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "low" => Ok(SampleRate::Low),
            "medium" => Ok(SampleRate::Medium),
            "high" => Ok(SampleRate::High),
            _ => Err(format!("Unknown sample_rate: {}", value)),
        }
    }
}

impl Serialize for SampleRate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SampleRate::Low => serializer.serialize_u16(8000),
            SampleRate::Medium => serializer.serialize_u16(24000),
            SampleRate::High => serializer.serialize_u16(48000),
        }
    }
}
