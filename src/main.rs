use csv;
use strum;
use structopt;

mod escape_str;
mod file_with_progress_bar;
mod generate_xml;
mod record_type;

use crate::{
    file_with_progress_bar::FileWithProgressBar,
    generate_xml::generate_xml,
    record_type::RecordType
};
use quicli::prelude::*;
use std::{fs::File, io};
use structopt::StructOpt;

/// Reads csv and writes xml. The resulting XML Document is intended for deliveries to the
/// Blue Yonder Supply and Demand API. This tool only checks for correct utf8 encoding and nothing
/// else.
#[derive(Debug, StructOpt)]
struct Cli {
    /// Root tag of generated XML.
    #[structopt()]
    category: String,
    /// Path to input file. If ommited STDIN is used for input.
    #[structopt(long = "input", short = "i", parse(from_os_str))]
    input: Option<std::path::PathBuf>,
    /// Path to output file. If ommited output is written to STDOUT.
    #[structopt(long = "output", short = "o", parse(from_os_str))]
    output: Option<std::path::PathBuf>,
    /// Record type of generated XML. Should be either Record, DeleteRecord, DeleteAllRecords.
    #[structopt(long = "record-type", short = "r", default_value = "Record")]
    record_type: RecordType,
    /// Character used as delimiter between csv columns. While this tool assumes utf8 encoding,
    /// only ASCII delimiters are supported.
    #[structopt(long = "delimiter", short = "d", default_value = ",")]
    delimiter: char,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    let input: Box<dyn io::Read> = if let Some(input) = args.input {
        Box::new(FileWithProgressBar::new(File::open(&input)?)?)
    } else {
        Box::new(io::stdin())
    };
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(args.delimiter as u8)
        .from_reader(input);

    let mut out: Box<dyn io::Write> = if let Some(output) = args.output {
        Box::new(io::BufWriter::new(File::create(&output)?))
    } else {
        Box::new(io::stdout())
    };
    generate_xml(&mut out, &mut reader, &args.category, args.record_type)?;
    Ok(())
}
