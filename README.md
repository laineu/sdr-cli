# sdr-cli
steam datagram relay manager, mainly used for region picking

## notice
* only linux
* only firewalld
* only tf2 and cs2 tested
* only usa whitelist currently

## usage
these examples will output a command that you can use to blacklist all ips that do not fit the whitelist

whitelisting usa cs2/tf2 servers:
```bash
cargo run -- cs2 firewalld usa
cargo run -- tf2 firewalld usa
```

whitelisting usa by appid (ex: dota2):
```bash
cargo run -- 570 firewalld usa
```

# reverting
as this curently only supports firewalld, you could either restart firewalld:
```bash
systemctl restart firewalld
```
or reboot your system
