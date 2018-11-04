#[macro_use]
extern crate clap;
extern crate humansize;
extern crate num_cpus;
extern crate rayon;

mod walk;

use std::path::PathBuf;

use clap::{App, AppSettings, Arg};
use humansize::{file_size_opts, FileSize};

use walk::Walk;

fn print_result(size: u64) {
    println!(
        "{} ({} bytes)",
        size.file_size(file_size_opts::DECIMAL).unwrap(),
        size
    );
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
            Arg::with_name("threads")
                .long("threads")
                .short("j")
                .value_name("N")
                .takes_value(true)
                .help("Set the number of threads (default: 3 x num cores)"),
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

    let paths = &[PathBuf::from(".")];
    let walk = Walk::new(paths, num_threads);
    let size = walk.run();
    print_result(size);
}
