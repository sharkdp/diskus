use std::path::PathBuf;

use clap::{crate_name, crate_version, App, AppSettings, Arg};
use humansize::file_size_opts::{self, FileSizeOpts};
use humansize::FileSize;
use num_format::{Locale, ToFormattedString};

use diskus::{Error, FilesizeType, Walk};

fn print_result(size: u64, errors: &[Error], size_format: &FileSizeOpts, verbose: bool) {
    if verbose {
        for err in errors {
            match err {
                Error::NoMetadataForPath(path) => {
                    eprintln!(
                        "diskus: could not retrieve metadata for path '{}'",
                        path.to_string_lossy()
                    );
                }
                Error::CouldNotReadDir(path) => {
                    eprintln!(
                        "diskus: could not read contents of directory '{}'",
                        path.to_string_lossy()
                    );
                }
            }
        }
    } else if !errors.is_empty() {
        eprintln!(
            "[diskus warning] the results may be tainted. Re-run with -v/--verbose to print all errors."
        );
    }

    if atty::is(atty::Stream::Stdout) {
        println!(
            "{} ({:} bytes)",
            size.file_size(size_format).unwrap(),
            size.to_formatted_string(&Locale::en)
        );
    } else {
        println!("{}", size);
    }
}

fn main() {
    let app = App::new(crate_name!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .version(crate_version!())
        .about("Compute disk usage for the given filesystem entries")
        .arg(
            Arg::with_name("path")
                .multiple(true)
                .help("List of filesystem paths"),
        )
        .arg(
            Arg::with_name("threads")
                .long("threads")
                .short("j")
                .value_name("N")
                .takes_value(true)
                .help("Set the number of threads (default: 3 x num cores)"),
        )
        .arg(
            Arg::with_name("size-format")
                .long("size-format")
                .takes_value(true)
                .value_name("type")
                .possible_values(&["decimal", "binary"])
                .default_value("decimal")
                .help("Output format for file sizes (decimal: MB, binary: MiB)"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .takes_value(false)
                .help("Do not hide filesystem errors"),
        );

    #[cfg(not(windows))]
    let app = app.arg(
        Arg::with_name("apparent-size")
            .long("apparent-size")
            .short("b")
            .help("Compute apparent size instead of disk usage"),
    );

    let matches = app.get_matches();

    // Setting the number of threads to 3x the number of cores is a good tradeoff between
    // cold-cache and warm-cache runs. For a cold disk cache, we are limited by disk IO and
    // therefore want the number of threads to be rather large in order for the IO scheduler to
    // plan ahead. On the other hand, the number of threads shouldn't be too high for warm disk
    // caches where we would otherwise pay a higher synchronization overhead.
    let num_threads = matches
        .value_of("threads")
        .and_then(|t| t.parse().ok())
        .unwrap_or(3 * num_cpus::get());

    let paths: Vec<PathBuf> = matches
        .values_of("path")
        .map(|paths| paths.map(PathBuf::from).collect())
        .unwrap_or_else(|| vec![PathBuf::from(".")]);

    let filesize_type = if matches.is_present("apparent-size") {
        FilesizeType::ApparentSize
    } else {
        FilesizeType::DiskUsage
    };

    let size_format = match matches.value_of("size-format") {
        Some("decimal") => file_size_opts::DECIMAL,
        _ => file_size_opts::BINARY,
    };

    let verbose = matches.is_present("verbose");

    let walk = Walk::new(&paths, num_threads, filesize_type);
    let (size, errors) = walk.run();
    print_result(size, &errors, &size_format, verbose);
}
