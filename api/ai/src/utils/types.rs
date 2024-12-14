use serde::Serialize;

#[derive(Serialize)]
pub struct Percentage(u8);

impl Percentage {
    pub fn new(value: u8) -> Result<Percentage, String> {
        Percentage::try_from(value)
    }

    pub fn new_or_default(value: Option<u8>) -> Percentage {
        match value {
            None => Percentage::default(),
            Some(v) => Percentage::new(v).unwrap_or_default(),
        }
    }
}

impl TryFrom<u8> for Percentage {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 100 {
            return Err("Percentage must be between 0 and 100".to_string());
        }

        Ok(Percentage(value))
    }
}

impl Default for Percentage {
    fn default() -> Self {
        Percentage(50)
    }
}
