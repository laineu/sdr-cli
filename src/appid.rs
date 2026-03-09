use std::str::FromStr;

#[derive(Debug)]
pub enum AppId {
    App(u64),
}

impl AppId {
    pub fn get_appid(&self) -> u64 {
        match self {
            AppId::App(id) => *id,
        }
    }

    // TODO: move this somewhere else
    pub fn get_sdr_url(&self) -> String {
        format!(
            "https://api.steampowered.com/ISteamApps/GetSDRConfig/v1?appid={}",
            self.get_appid()
        )
    }
}

impl FromStr for AppId {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tf2" => Ok(AppId::App(440)),
            "cs2" => Ok(AppId::App(730)),
            other => other
                .parse::<u64>()
                .map(Self::App)
                .map_err(|_| "must be either: tf2, cs2 or a number"),
        }
    }
}
