# Daktronics Singular UI

> A generically named cross-platform desktop application enabling free streaming
> of data from a Daktronics console to the SaaS platform Singular.Live.

<a href="https://github.com/iced-rs/iced">
  <img src="https://gist.githubusercontent.com/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width="130px">
</a>

## Setup

To build from source, make sure you have a Rust toolchain installed, then do:

```bash
cargo build --release
```

and the binary will be built at `target/release/daktronics-singular-ui`. The
binary does not have any dependencies as everything (libraries, icons, etc.) is
statically linked.

## Stack

- [Iced](https://github.com/iced-rs/iced)
- [reqwest](https://github.com/seanmonstar/reqwest)
- [daktronics-allsport-5000-rs](https://github.com/zabackary/daktronics-allsport-5000-rs)
- [Rust](https://www.rust-lang.org/)

## About

Written for the [Christian Academy in Japan productions](https://caj.ac.jp/live)
team for use in live-streaming sports events.
