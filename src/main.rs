pub mod appid;
pub mod firewall;
pub mod preset;

use crate::{appid::AppId, firewall::Firewall, preset::Preset};
use serde::Deserialize;
use std::{collections::HashMap, env::Args, fmt::Debug, str::FromStr};

pub fn parse_arg<T>(args: &mut Args, name: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    args.next()
        .unwrap_or_else(|| panic!("{}: missing", name))
        .parse::<T>()
        .unwrap_or_else(|e| panic!("{}: failed to parse, err: {:?}", name, e))
}

#[derive(Debug, Deserialize)]
pub struct Relay {
    pub ipv4: String,
}

#[derive(Debug, Deserialize)]
pub struct Pop {
    pub relays: Option<Vec<Relay>>,
}

#[derive(Debug, Deserialize)]
pub struct SDR {
    pub pops: HashMap<String, Pop>,
}

fn main() {
    let mut args = std::env::args();
    // program name, discard it
    let _ = parse_arg::<String>(&mut args, "self");

    let appid = parse_arg::<AppId>(&mut args, "appid");
    let firewall = parse_arg::<Firewall>(&mut args, "firewall");
    let preset = parse_arg::<Preset>(&mut args, "preset");

    // get sdr and parse
    let sdr = reqwest::blocking::get(appid.get_sdr_url())
        .expect("invalid sdr response")
        .json::<SDR>()
        .expect("unable to parse SDR");

    // sdr -> (shortname, [relays])
    let relays: HashMap<String, Vec<Relay>> = sdr
        .pops
        .into_iter()
        .map(|(name, pop)| (name, pop.relays.unwrap_or_default()))
        .filter(|(_, pop)| !pop.is_empty())
        .collect();

    // filter out everything thats not preset
    let to_block: Vec<Relay> = relays
        .into_iter()
        .filter(|(name, _)| !preset.get_preset().iter().any(|p| p.starts_with(name)))
        .flat_map(|(_, relay)| relay)
        .collect();

    // generate cmd
    // TODO: make this work on other platforms
    let block_cmd: Vec<String> = to_block.iter().map(|r| firewall.block(&r.ipv4)).collect();
    let block_cmd = format!("({})", block_cmd.join("&&"));

    println!("{}", block_cmd);
}
