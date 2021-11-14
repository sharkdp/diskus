# unreleased

## Changes


## Features


## Bugfixes


## Other


## Packaging

# v0.6.0

- Updated dependencies

# v0.6.0

## Changes

There is an important change in default behavior: `diskus` will now report "disk usage" instead of "apparent file size", in analogy to what `du -sh` does.

At the same time however, we introduce a new `-b`/`--apparent-size` option which can be used to switch back to apparent file size (in analogy to what `du -sbh` does).

see #25

## Features

- `diskus` is now available for Windows, see #32 (@fawick)
- Error messages are now hidden by default and can be re-enabled via `--verbose`, see #34 (@wngr)
- Added a new `--size-format <type>` option which can be used to switch from decimal to binary exponents (MiB instead of MB).
- `diskus` changes its output format when the output is piped to a file or to another program. It will simply print the number of bytes, see #35
- Added a new `-b`/`--apparent-size` option which can be used to switch from "disk usage" to "apparent size" (not available on Windows)

## Other

- diskus is now in the official Arch repositories, see #24 (@polyzen)
- diskus is now available on NixOS, see #26 (@fuerbringer)
- diskus is now available on Homebrew and MacPorts, see #33 (@heimskr)
- Added a man page

# v0.5.0

- Expose diskus internals as a library, see #21 (@amilajack)

# v0.4.0

- More performance improvements by using a custom parallel directory-walker, see #15

# v0.3.1

# v0.3.0

- Renamed the project to diskus

# v0.2.0

- Fine-tuned number of threads (makes is even faster)

# v0.1.0

Initial release

