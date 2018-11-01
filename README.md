# dup

[![Build Status](https://travis-ci.org/sharkdp/dup.svg?branch=master)](https://travis-ci.org/sharkdp/dup)

*A minimal, fast alternative to `du -sh`.*

`dup` is a very simple program that computes the total size of the current directory. It is a
parallelized version of `du -sh`. On my 8-core laptop, it is about nine times faster than `du` for
a cold disk cache and more than twice as fast on a warm disk cache.

``` bash
> dup
14.56 GB (14556806983 bytes)
```

## Benchmark

The following benchmarks have been performed with [hyperfine](https://github.com/sharkdp/hyperfine) on
a moderately large folder (15GB, 100k directories, 400k files). Smaller folders are not really of any
interest since all programs would finish in a reasonable time that would not interrupt your workflow.

In addition to `du` and `dup`, we also add [tin-summer](https://github.com/vmchale/tin-summer) (`sn`) and
[`dust`](https://github.com/bootandy/dust) in our comparison. Both are also written in Rust and provide
much more features than `dup` (check them out!). The optimal number of threads for `sn` (`-j` option) was
determined via `hyperfine --parameter-scan`.

### Cold disk cache

```bash
sudo -v
hyperfine --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches' \
    'dup' 'sn p -d0 -j8' 'du -sb' 'dust -d0'
```
(the `sudo`/`sync`/`drop_caches` commands are a way to
[clear the filesystem caches between benchmarking runs](https://github.com/sharkdp/hyperfine#io-heavy-programs))

| Command | Mean [s] | Min…Max [s] |
|:---|---:|---:|
| `dup` | 1.729 ± 0.012 | 1.717…1.756 |
| `sn p -d0 -j8` | 9.778 ± 0.098 | 9.587…9.904 |
| `du -sb` | 16.016 ± 0.067 | 15.923…16.147 |
| `dust -d0` | 19.845 ± 0.466 | 19.428…20.948 |

### Warm disk cache

On a warm disk cache, the differences are smaller:
```bash
hyperfine --warmup 5 'dup' 'sn p -d0 -j8' 'du -sb' 'dust -d0'
```

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `dup` | 465.9 ± 14.7 | 446.5…487.4 |
| `sn p -d0 -j8` | 596.4 ± 12.2 | 579.2…615.9 |
| `du -sb` | 1100.3 ± 20.5 | 1086.9…1153.0 |
| `dust -d0` | 3560.1 ± 27.8 | 3521.7…3612.8 |

## Installation

### On Debian-based systems

``` bash
wget "https://github.com/sharkdp/dup/releases/download/v0.2.0/dup_0.2.0_amd64.deb"
sudo dpkg -i dup_0.2.0_amd64.deb
```

### On other distrubutions

Check out the [release page](https://github.com/sharkdp/dup/releases) for binary builds.

### Via cargo

```
cargo install du-dup
```
