use std::str::FromStr;

#[derive(Debug)]
pub enum Firewall {
    FirewallD,
}

impl Firewall {
    fn block_firewalld(ip: &str) -> String {
        let output = format!(
            "firewall-cmd --direct --add-rule ipv4 filter OUTPUT 0 -d {} -j DROP",
            ip
        );
        format!("({})", output)
    }

    pub fn block(&self, ip: &str) -> String {
        match self {
            Self::FirewallD => Firewall::block_firewalld(ip),
        }
    }
}

impl FromStr for Firewall {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "firewalld" => Ok(Self::FirewallD), // linux
            _ => Err("unsupported"),
        }
    }
}
