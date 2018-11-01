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

In addition to `du` and `dup`, we also add [tin-summer](https://github.com/vmchale/tin-summer) (`sn`) in
our comparison. It is a fully-featured replacement for (alternative to) `du` written in Rust, which you
should check out. The optimal number of threads for `sn` (`-j` option) was determined
via `hyperfine --parameter-scan`.

### Cold disk cache

```bash
sudo -v
hyperfine --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches' \
    'dup' 'sn p -d0 -j8' 'du -shb'
```
(the `sudo`/`sync`/`drop_caches` commands are a way to
[clear the filesystem caches between benchmarking runs](https://github.com/sharkdp/hyperfine#io-heavy-programs))

| Command | Mean [s] | Min…Max [s] |
|:---|---:|---:|
| `dup` | 3.206 ± 0.026 | 3.170…3.241 |
| `sn p -d0 -j8` | 9.799 ± 0.063 | 9.700…9.909 |
| `du -shb` | 16.262 ± 0.161 | 16.056…16.552 |


### Warm disk cache

On a warm disk cache, the differences are smaller. But I believe that in most situations where you are interested
in total disk usage, you have a cold disk cache.

```bash
hyperfine --warmup 5 'dup' 'sn p -d0 -j8' 'du -shb'
```

| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `dup` | 413.6 ± 3.8 | 405.9…420.5 |
| `sn p -d0 -j8` | 613.6 ± 11.7 | 602.0…633.0 |
| `du -shb` | 1112.2 ± 4.2 | 1104.9…1118.4 |
