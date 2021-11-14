# diskus

[![CICD](https://github.com/sharkdp/diskus/actions/workflows/CICD.yml/badge.svg)](https://github.com/sharkdp/diskus/actions/workflows/CICD.yml)

*A minimal, fast alternative to `du -sh`.*

`diskus` is a very simple program that computes the total size of the current directory. It is a
parallelized version of `du -sh`. On my 8-core laptop, it is about ten times faster than `du` with
a cold disk cache and more than three times faster with a warm disk cache.

``` bash
> diskus
9.59 GB (9,587,408,896 bytes)
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
    'diskus' 'du -sh' 'sn p -d0 -j8' 'dust -d0'
```
(the `sudo`/`sync`/`drop_caches` commands are a way to
[clear the filesystem caches between benchmarking runs](https://github.com/sharkdp/hyperfine#io-heavy-programs))

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `diskus` | 1.746 ± 0.017 | 1.728 | 1.770 | 1.00 |
| `du -sh` | 17.776 ± 0.549 | 17.139 | 18.413 | 10.18 |
| `sn p -d0 -j8` | 18.094 ± 0.566 | 17.482 | 18.579 | 10.36 |
| `dust -d0` | 21.357 ± 0.328 | 20.974 | 21.759 | 12.23 |


### Warm disk cache

On a warm disk cache, the differences are smaller:
```bash
hyperfine --warmup 5 'diskus' 'du -sh' 'sn p -d0 -j8' 'dust -d0'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `diskus` | 500.3 ± 17.3 | 472.9 | 530.6 | 1.00 |
| `du -sh` | 1098.3 ± 10.0 | 1087.8 | 1122.4 | 2.20 |
| `sn p -d0 -j8` | 1122.2 ± 18.2 | 1107.3 | 1170.1 | 2.24 |
| `dust -d0` | 3532.1 ± 26.4 | 3490.0 | 3563.1 | 7.06 |


## Installation

### On Debian-based systems

You can download the latest Debian package from the
[release page](https://github.com/sharkdp/diskus/releases) and install it via `dpkg`:

``` bash
wget "https://github.com/sharkdp/diskus/releases/download/v0.7.0/diskus_0.7.0_amd64.deb"
sudo dpkg -i diskus_0.7.0_amd64.deb
```

### On Arch-based systems

``` bash
pacman -S diskus
```

Or download [diskus-bin](https://aur.archlinux.org/packages/diskus-bin/) from the AUR.

### On Void-based systems

``` bash
xbps-install diskus
```

### On macOS

You can install `diskus` with [Homebrew](https://formulae.brew.sh/formula/diskus):
```
brew install diskus
```

Or with [MacPorts](https://ports.macports.org/port/diskus/summary):
```
sudo port install diskus
```

### On Haiku

``` bash
pkgman install diskus
```

### On NixOS

```
nix-env -iA nixos.diskus
```

Or add it to `environment.systemPackages` in your `configuration.nix`.

### On other systems

Check out the [release page](https://github.com/sharkdp/diskus/releases) for binary builds.

### Via cargo

If you have Rust 1.50 or higher, you can install `diskus` from source via `cargo`:
```
cargo install diskus
```

## Windows caveats

Windows-internal tools such as Powershell, Explorer or `dir` are not respecting hardlinks or
junction points when determining the size of a directory. `diskus` does the same and counts
such entries multiple times (on Unix systems, multiple hardlinks to a single file are counted
just once).

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
