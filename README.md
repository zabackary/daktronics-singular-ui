# Daktronics Singular UI

> A generically named cross-platform desktop application enabling free streaming
> of data from a Daktronics console to the SaaS platform Singular.Live.

<a href="https://github.com/iced-rs/iced">
  <img src="https://gist.githubusercontent.com/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width="130px">
</a>

## Demo video

[daktronics-singular-ui-demo.webm](https://github.com/user-attachments/assets/473b455a-af18-4af5-89aa-710fdfc68522)

## Setup

### Binaries

Grab the binaries from the [GitHub releases page](https://github.com/zabackary/daktronics-singular-ui/releases).
For now, a Linux binary and Windows binary are provided. Daktronics Singular UI
is highly portable, so a MacOS version is possible. However, [Apple does not make
cross-compiling easy](https://users.rust-lang.org/t/is-cross-compile-from-linux-to-mac-supported/95105)
and I don't own a MacOS machine (I do have an old Mac Mini, so I might turn that
into a build server, but no promises).

### From source

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
