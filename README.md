# Daktronics Singular UI

> A generically named cross-platform desktop application enabling free streaming
> of data from a Daktronics console to the SaaS platform Singular.Live.

<a href="https://github.com/iced-rs/iced">
  <img src="https://gist.githubusercontent.com/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width="130px">
</a>

## Demo video

[daktronics-singular-ui-demo.webm](https://github.com/user-attachments/assets/473b455a-af18-4af5-89aa-710fdfc68522)

## Usage

Follow the directions in the app in order to use DSU. Basically, you need to
create a _profile_ which specifies a mapping between the fields received from
the Daktronics console (see
[the documentation](https://docs.rs/daktronics-allsport-5000/latest/daktronics_allsport_5000/sports/index.html)
for fields available per sport). This mapping is used to read the incoming
fields and output them as JSON for the Singular.Live datastream. For ease of
use, a template Singular.Live Composition Script is bundled in the app; see the
"Set up" tab.

Daktronics Singular UI also supports a variety of command-line options for use
as a server application. As of September 2024, using DSU without a UI (i.e.
headless) mode is not supported, but other than that, it should be all set to be
used in an environment without human intervention. An example command can be
found below.

```
daktronics-singular-ui \
  --profile /path/to/profile.dsu \ # the path to the profile
  --start \                        # automatically start streaming
  --serial-path /dev/USB0 \        # which serial port to use
  --hide-header \                  # hide the app header for small screens
  --unattended \                   # automatically restart on errors (default 3)
  --fullscreen                     # fullscreen UI
```

See the output of `daktronics-singular-ui --help`.

```
$ ./daktronics-singular-ui --help
Links the output of an Daktronics AllSport 5000 to Singular.Live

Usage: daktronics-singular-ui [OPTIONS]

Options:
  -l, --headless
          Whether to hide the UI (run without a window). Must be used with --start

  -p, --profile <PROFILE>
          The profile configuration file path. If not provided, the UI will prompt for one

  -s, --start
          Whether to start the stream immediately. Must be used with --profile and --serial-path

  -e, --serial-path <SERIAL_PATH>
          What serial path (e.g. /dev/xxx or COM1 on Windows) to use, when --start is used

  -f, --fullscreen
          Whether to start the program in fullscreen mode

      --hide-header
          Whether to hide the header and show a minimized video

          Useful for small display sizes

  -u, --unattended
          Enable unattended mode, restarting automatically if there are many errors

          Passing a number indicates the maximum tolerated error count. Default 3. Max 15.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

## Setup

### Binaries

Grab the binaries from the
[GitHub releases page](https://github.com/zabackary/daktronics-singular-ui/releases).
For now, a Linux binary and Windows binary are provided. Daktronics Singular UI
is highly portable, so a MacOS version is possible. However,
[Apple does not make cross-compiling easy](https://users.rust-lang.org/t/is-cross-compile-from-linux-to-mac-supported/95105)
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

To cross compile, using `cross` should be supported.

## Stack

- [Iced](https://github.com/iced-rs/iced)
- [reqwest](https://github.com/seanmonstar/reqwest)
- [daktronics-allsport-5000-rs](https://github.com/zabackary/daktronics-allsport-5000-rs)
- [Rust](https://www.rust-lang.org/)

## About

Written for the [Christian Academy in Japan productions](https://caj.ac.jp/live)
team for use in live-streaming sports events.
