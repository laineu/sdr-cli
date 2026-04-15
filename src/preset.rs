use std::str::FromStr;

#[derive(Debug)]
pub enum Preset {
    USA,
    China,
    Australia,
    Japan,
    Korea,
    Europe,
}

impl Preset {
    pub fn get_preset(&self) -> &[&str] {
        match self {
            Preset::USA => PRESET_USA,
            Preset::China => PRESET_CHINA,
            Preset::Australia => PRESET_AUSTRALIA,
            Preset::Japan => PRESET_JAPAN,
            Preset::Korea => PRESET_KOREA,
            Preset::Europe => PRESET_EUROPE,
        }
    }
}

impl FromStr for Preset {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "usa" => Ok(Self::USA),
            "china" => Ok(Self::China),
            "australia" => Ok(Self::Australia),
            "japan" => Ok(Self::Japan),
            "korea" => Ok(Self::Korea),
            "europe" => Ok(Self::Europe),
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

pub const PRESET_CHINA: &[&str] = &[
    "ctu",  // Alibaba Cloud Chengdu (China)
    "hkg",  // Hong Kong
    "pek",  // Alibaba Cloud Beijing (China)
    "pvg",  // Alibaba Cloud Shanghai (China)
    "pwg",  // Perfect World Guangdong 1 (China)
    "pwj",  // Perfect World Tianjin (China)
    "pwu",  // Perfect World Hebei (China)
    "pww",  // Perfect World Wuhan (China)
    "pwz",  // Perfect World Zhejiang (China)
    "sha",  // Perfect World Shanghai (China)
    "shb",  // Perfect World (sha-4) Backbone (Shanghai, China)
    "tgd",  // Tencent Guangzhou (China)
    "ctum", // Alibaba Cloud Chengdu - Mobile (China)
    "pekm", // Alibaba Cloud Beijing - Mobile (China)
    "pvgm", // Alibaba Cloud Shanghai - Mobile (China)
    "tgdm", // Tencent Guangzhou - Mobile (China)
    "ctut", // Alibaba Cloud Chengdu - Telecom (China)
    "pekt", // Alibaba Cloud Beijing - Telecom (China)
    "pvgt", // Alibaba Cloud Shanghai - Telecom (China)
    "tgdt", // Tencent Guangzhou - Telecom (China)
    "ctuu", // Alibaba Cloud Chengdu - Unicom (China)
    "peku", // Alibaba Cloud Beijing - Unicom (China)
    "pvgu", // Alibaba Cloud Shanghai - Unicom (China)
    "tgdu", // Tencent Guangzhou - Unicom (China)
];

pub const PRESET_AUSTRALIA: &[&str] = &[
    "syd", // Sydney (Australia)
];

pub const PRESET_JAPAN: &[&str] = &[
    "tyo", // Tokyo Koto City (Japan)
];

pub const PRESET_KOREA: &[&str] = &[
    "seo", // Seoul (South Korea)
];

pub const PRESET_EUROPE: &[&str] = &[
    "ams",  // Amsterdam (Netherlands)
    "fra",  // Frankfurt (Germany)
    "fsn",  // Falkenstein (Germany)
    "hel",  // Helsinki (Finland)
    "lhr",  // London (England)
    "mad",  // Madrid (Spain)
    "par",  // Paris (France)
    "sto",  // Stockholm - Kista (Sweden)
    "vie",  // Vienna (Austria)
    "waw",  // Warsaw (Poland)
    "sto2", // Stockholm - Bromma (Sweden)
];
