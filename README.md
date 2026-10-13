# sdr-cli
steam datagram relay manager, mainly used for region picking

## notice
* only linux
* only firewalld
* only tf2 and cs2 tested
* whitelist based

## list of presets
```
na -> North America
cn -> China
au -> Australia
jp -> Japan
kr -> Korea
eu -> Europe
sa -> South America
in -> India
me -> Middle East
af -> Africa
```

## usage
these examples will output a command that you can use to blacklist all ips that do not fit the whitelist

whitelisting cs2/tf2 servers based in north america:
```bash
cargo run -q -- cs2 firewalld na
cargo run -q -- tf2 firewalld na
```

whitelisting north america by appid (ex: dota2):
```bash
cargo run -q -- 570 firewalld na
```

# reverting
as this curently only supports firewalld, you could either restart firewalld:
```bash
systemctl restart firewalld
```
or reboot your system
