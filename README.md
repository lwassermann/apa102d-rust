# apa102d (rust)

This is a version of [apa102d](https://github.com/schoeppel/apa102d) written in [rust](https://www.rust-lang.org/en-US/). They share a configuration file, but might listen to different mqtt-topics.

## Configuration

* Add `/etc/apa102d.conf` based on `config/apa102d.conf.sample`
* Add hosts-entry for the mqtt server `mqtt`
* Make sure `spi` is enabled, i.e. `/dev/spidev0.0` exists. [RapsberryPI](https://www.raspberrypi.org/documentation/hardware/raspberrypi/spi/README.md)

## Compilation

1. Install rust
2. `cargo build`

## Development

- Tests: `cargo test`
- This project uses [`rustfmt`](https://github.com/rust-lang-nursery/rustfmt) for formating the sources (`cargo fmt`)

## Next steps
* [ ] Get a light to show up
* [ ] Receive an mqtt message
* [ ] Implement the test effect
* [ ] Subscribe/Publish topics
* [ ] Parse the config file
* [ ] Implement the particles effect?
* [ ] Letters?

## Troubleshooting

* `error: cannot find macro ```ioctl!``` in this scope`
This crate depends on `spidev`, which can only be compiled in a Linux environment. You'll need to cross-compile for a Linux machine ([1](https://github.com/Ogeon/rust-on-raspberry-pi) or [2](https://akappel.github.io/2017/11/07/rpi-crosstool.html)?).
