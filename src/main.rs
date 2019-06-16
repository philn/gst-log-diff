extern crate difference;
extern crate gst_log_parser;
extern crate regex;
extern crate structopt;
extern crate structopt_derive;
extern crate term;

use difference::{Changeset, Difference};
use gst_log_parser::{parse, Entry};
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "gst-log-diff",
    about = "Print the difference between two GStreamer log files"
)]
struct Opt {
    #[structopt(
        short = "c",
        long = "category",
        help = "Filter on given category. A regular expression can be provided"
    )]
    category: Option<String>,
    #[structopt(parse(from_os_str), help = "Input file1")]
    input1: PathBuf,
    #[structopt(parse(from_os_str), help = "Input file2")]
    input2: PathBuf,
}

fn category_filter(re: &Option<Regex>, entry1: &Entry, entry2: &Entry) -> bool {
    match re {
        Some(re) => re.is_match(&entry1.category) && re.is_match(&entry2.category),
        None => true,
    }
}

fn main() {
    let opt = Opt::from_args();
    let f1 = File::open(opt.input1).expect("Failed to open log file");
    let f2 = File::open(opt.input2).expect("Failed to open log file");

    let mut t = term::stdout().unwrap();

    let category_regex = match opt.category {
        Some(pattern) => Some(Regex::new(&pattern).unwrap()),
        None => None,
    };
    let parsed1 = parse(f1);
    let mut parsed2 = parse(f2);
    for entry1 in parsed1 {
        if let Some(other) = parsed2.nth(0) {
            if category_filter(&category_regex, &entry1, &other) {
                let Changeset { diffs, .. } = Changeset::new(&entry1.message, &other.message, "\n");
                for item in &diffs {
                    match item {
                        Difference::Same(ref _x) => {
                            t.reset().unwrap();
                        }
                        Difference::Add(ref x) => {
                            t.reset().unwrap();
                            write!(t, "{} => ", other.ts).unwrap();
                            t.fg(term::color::GREEN).unwrap();
                            writeln!(t, "+{}", x).unwrap();
                        }
                        Difference::Rem(ref x) => {
                            t.reset().unwrap();
                            write!(t, "{} => ", entry1.ts).unwrap();
                            t.fg(term::color::RED).unwrap();
                            writeln!(t, "-{}", x).unwrap();
                        }
                    }
                }
            }
        }
    }
    t.reset().unwrap();
    t.flush().unwrap();
}
