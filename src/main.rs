use std::path::PathBuf;
use std::io::{self, Write};

use clap::{crate_name, crate_version, App, AppSettings, Arg};
use humansize::file_size_opts::{self, FileSizeOpts};
use humansize::FileSize;
use num_format::{Locale, ToFormattedString};
use tabwriter::TabWriter;

use diskus::{Error, FilesizeType, Walk};

fn build_message(path: Option<&PathBuf>, size: u64, errors: &[Error], size_format: &FileSizeOpts, raw: bool, verbose: bool) -> String {
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

    let path_info = path.map(|p| format!("\t{}", p.to_string_lossy())).unwrap_or_default();
    if raw {
        format!("{}{}", size, path_info)
    } else {
        let human_readable_size = size.file_size(size_format).unwrap();
        let size_in_bytes = size.to_formatted_string(&Locale::en);
        if verbose {
            format!("{} ({:} bytes){}", human_readable_size, size_in_bytes, path_info)
        } else {
            format!("{}{}", human_readable_size, path_info)
        }
    }
}


fn perform_walks(walks: Vec<Walk>, aggregate: bool, size_format: FileSizeOpts, raw: bool, verbose: bool) {
    if aggregate {
        let mut total_size = 0;
        let mut all_errors = Vec::new();

        for walk in walks {
            let (size, errors) = walk.run();
            total_size += size;
            all_errors.extend(errors);
        }

        println!("{}",
            build_message(None, total_size, &all_errors, &size_format, raw, verbose)
        );
    } else {
        let mut tw = TabWriter::new(io::stdout()).padding(2);
        for walk in walks {
            // each Walk knows its own root_directories
            let (size, errors) = walk.run();
            assert_eq!(walk.get_root_directories().len(), 1, "perform_walks can only be called without aggregation with a single root directory");
            let path = &walk.get_root_directories()[0];
            writeln!(tw, "{}",
                build_message(Some(path), size, &errors, &size_format, raw, verbose)
            ).unwrap();
        }
        tw.flush().unwrap();
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
            Arg::with_name("raw")
                .long("raw")
                .takes_value(false)
                .help("Instead of human-readable sizes uses raw numbers in bytes. Makes the system ignore the parameter \"size-format\"."),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .takes_value(false)
                .help("Do not hide filesystem errors"),
        )
        .arg(
            Arg::with_name("aggregate")
                .long("aggregate")
                .short("a")
                .takes_value(false)
                .help("Aggregate sizes across all provided paths"),
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

    let raw = matches.is_present("raw");
    let verbose = matches.is_present("verbose");
    let aggregate = matches.is_present("aggregate");
    let walks: Vec<Walk> = if aggregate {
        vec![Walk::new(&paths, num_threads, filesize_type)]
    } else {
        paths
            .iter()
            .map(|p| Walk::new(std::slice::from_ref(p), num_threads, filesize_type))
            .collect()
    };

    perform_walks(walks, aggregate, size_format, raw, verbose);
}
