# watchttp
(macOS ONLY)

Watch a list of webpages for changes at a set interval. Send system notifcations when they change.

![image of macOS notification](./images/notif.png)


## Config Format
```yaml
sites:
  - https://n8ta.com
  - https://archive.org
period_ms: 3600000
```

## Usage
```shell
cargo build --release
./target/release/watchttp ./config.yaml
```