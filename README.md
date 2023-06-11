## Overview

This is an updater program for [DuckDNS](https://www.duckdns.org/) written in Rust. See the [DuckDNS spec](https://www.duckdns.org/spec.jsp) for more information.

### Why?

Why not use curl? Because this is a hobby project and I want to learn more about Rust.

## Compiling

Use `cargo build` after the project is cloned to your local machine.

```text
cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
```

The finished build is at `target/debug/duckdns-updater`.

## Usage

The program expects two arguments: The urls to update and an update token.

```text
duckdns-updater url1,url2 myToken
```

The DuckDNS spec suggests `KO` is expected for bad requests. For successful requests `OK` followed by some debug output is seen as the `verbose=true` parameter is added to the request.
