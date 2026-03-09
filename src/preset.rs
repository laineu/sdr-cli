use std::str::FromStr;

#[derive(Debug)]
pub enum Preset {
    USA,
}

impl Preset {
    pub fn get_preset(&self) -> &[&str] {
        match self {
            Preset::USA => PRESET_USA,
        }
    }
}

impl FromStr for Preset {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "usa" => Ok(Self::USA),
            _ => Err("unsupported"),
        }
    }
}

pub const PRESET_USA: &[&str] = &[
    "atl", // Atlanta (Georgia)
    "dfw", // Dallas (Texas)
    "iad", // Sterling (Virginia)
    "lax", // Los Angeles (California)
    "ord", // Chicago (Illinois)
    "sea", // Seattle (Washington)
    "eat", // Wenatchee (Washington)
];
