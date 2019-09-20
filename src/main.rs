#[macro_use]
extern crate clap;
extern crate crossbeam_channel;
extern crate humansize;
extern crate num_cpus;
extern crate rayon;

mod walk;

use std::path::PathBuf;

use clap::{App, AppSettings, Arg};
use humansize::{file_size_opts, FileSize};

use walk::Walk;

fn print_result(size: u64, errors: &[walk::Err], verbose: bool) {
    let tainted = errors.iter().any(|x| {
        if let walk::Err::NoMetadataForPath(_) = x {
            true
        } else {
            false
        }
    });
    println!(
        "{} ({} bytes)",
        size.file_size(file_size_opts::DECIMAL).unwrap(),
        size
    );
    if verbose {
        for err in errors {
            match err {
                walk::Err::NoMetadataForPath(path) => {
                    eprintln!(
                        "diskus: could not retrieve metadata for path '{}'",
                        path.to_string_lossy()
                    );
                }
                walk::Err::CouldNotReadDir(path) => {
                    eprintln!(
                        "diskus: could not read contents of directory '{}'",
                        path.to_string_lossy()
                    );
                }
            }
        }
    } else if tainted {
        println!("Warning, results may be tainted. Try running with --verbose.");
    }
}

fn main() {
    let app = App::new(crate_name!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .version(crate_version!())
        .about("Compute disk usage for the current directory")
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
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .takes_value(false)
                .help("Emits verbose output"),
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

    let verbose = matches.is_present("verbose");

    let walk = Walk::new(&paths, num_threads);
    let (size, errors) = walk.run();
    print_result(size, &errors, verbose);
}
