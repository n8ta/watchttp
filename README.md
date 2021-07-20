# watchttp
(macOS ONLY)

Watch a list of webpages for changes at a set interval. Send system notifcations when they change.

![image of macOS notification](./images/notif.png)


## Config Format
Uses a config.yaml file located anywhere to hot-reload settings. Just launch watchttp as a daemon and edit the config whenever you need to.

`period_ms` is how long watchttp sleeps between loading each of your sites.
```yaml
sites:
  - https://n8ta.com
  - https://archive.org
period_ms: 3600000
```

## Crontab Installation
```shell
cargo build --release
cp ./target/release/watchttp /usr/local/bin
crontab -e
  // Add this line: (launches at boot)
  @reboot /usr/local/bin/watchttp /PATH/TO/CONFIG.YAML
```