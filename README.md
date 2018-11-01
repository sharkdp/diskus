# dup

[![Build Status](https://travis-ci.org/sharkdp/dup.svg?branch=master)](https://travis-ci.org/sharkdp/dup)

*A minimal, fast alternative to `du -sh`.*

`dup` is a very simple program that computes the total filesize of the current directory.
It is a parallelized version of `du -sh` or rather `du -sh --bytes`. On my 8-core laptop,
it is about five times faster than `du`.

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
| `dup` | 3.212 ± 0.030 | 3.185…3.276 |
| `sn p -d0 -j8` | 9.747 ± 0.089 | 9.646…9.908 |
| `du -sb` | 16.001 ± 0.091 | 15.854…16.181 |
| `dust -d0` | 19.921 ± 0.354 | 19.508…20.613 |

### Warm disk cache

On a warm disk cache, the differences are smaller. But I believe that in most situations where you are interested
in total disk usage, you have a cold disk cache.

```bash
hyperfine --warmup 5 'dup' 'sn p -d0 -j8' 'du -sb' 'dust -d0'
```

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `dup` | 414.4 ± 7.1 | 404.9…425.3 |
| `sn p -d0 -j8` | 606.6 ± 20.0 | 572.8…647.2 |
| `du -sb` | 1105.2 ± 13.5 | 1089.3…1129.9 |
| `dust -d0` | 3600.4 ± 23.5 | 3561.7…3649.5 |

## Installation

### On Debian-based systems

``` bash
wget "https://github.com/sharkdp/dup/releases/download/v0.1.0/dup_0.1.0_amd64.deb"
sudo dpkg -i dup_0.1.0_amd64.deb
```

### On other distrubutions

Check out the [release page](https://github.com/sharkdp/dup/releases) for binary builds.
