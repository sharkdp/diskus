# diskus

[![Build Status](https://travis-ci.org/sharkdp/diskus.svg?branch=master)](https://travis-ci.org/sharkdp/diskus)

*A minimal, fast alternative to `du -sh`.*

`diskus` is a very simple program that computes the total size of the current directory. It is a
parallelized version of `du -sh`. On my 8-core laptop, it is about ten times faster than `du` with
a cold disk cache and more than three times faster with a warm disk cache.

``` bash
> diskus
14.56 GB (14556806983 bytes)
```

## Benchmark

The following benchmarks have been performed with [hyperfine](https://github.com/sharkdp/hyperfine) on
a moderately large folder (15GB, 100k directories, 400k files). Smaller folders are not really of any
interest since all programs would finish in a reasonable time that would not interrupt your workflow.

In addition to `du` and `diskus`, we also add [tin-summer](https://github.com/vmchale/tin-summer) (`sn`) and
[`dust`](https://github.com/bootandy/dust) in our comparison. Both are also written in Rust and provide
much more features than `diskus` (check them out!). The optimal number of threads for `sn` (`-j` option) was
determined via `hyperfine --parameter-scan`.

### Cold disk cache

```bash
sudo -v
hyperfine --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches' \
    'diskus' 'sn p -d0 -j8' 'du -sb' 'dust -d0'
```
(the `sudo`/`sync`/`drop_caches` commands are a way to
[clear the filesystem caches between benchmarking runs](https://github.com/sharkdp/hyperfine#io-heavy-programs))

| Command | Mean [s] | Min…Max [s] |
|:---|---:|---:|
| `diskus` | 1.649 ± 0.009 | 1.640…1.663 |
| `sn p -d0 -j8` | 9.701 ± 0.067 | 9.598…9.828 |
| `du -sb` | 16.039 ± 0.069 | 15.918…16.152 |
| `dust -d0` | 19.769 ± 0.285 | 19.564…20.561 |


### Warm disk cache

On a warm disk cache, the differences are smaller:
```bash
hyperfine --warmup 5 'diskus' 'sn p -d0 -j8' 'du -sb' 'dust -d0'
```

| Command | Mean [s] | Min…Max [s] |
|:---|---:|---:|
| `diskus` | 0.314 ± 0.007 | 0.303…0.329 |
| `sn p -d0 -j8` | 0.622 ± 0.008 | 0.611…0.634 |
| `du -sb` | 1.130 ± 0.013 | 1.116…1.161 |
| `dust -d0` | 3.593 ± 0.057 | 3.544…3.743 |


## Installation

### On Debian-based systems

``` bash
wget "https://github.com/sharkdp/diskus/releases/download/v0.5.0/diskus_0.5.0_amd64.deb"
sudo dpkg -i diskus_0.5.0_amd64.deb
```

### On Arch-based systems

Download from the AUR: [diskus](https://aur.archlinux.org/packages/diskus/) or [diskus-bin](https://aur.archlinux.org/packages/diskus-bin/)

### On Void-based systems

``` bash
xbps-install diskus
```

### On Haiku

``` bash
pkgman install diskus
```

### On other systems

Check out the [release page](https://github.com/sharkdp/diskus/releases) for binary builds.

### Via cargo

If you have Rust 1.29 or higher, you can install `diskus` from source via `cargo`:
```
cargo install diskus
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
