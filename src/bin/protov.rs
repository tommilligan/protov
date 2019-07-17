use std::io::{self, Cursor, Read};

use hexyl::{BorderStyle, Printer};
use protobuf::descriptor::FileDescriptorSet;
use structopt::StructOpt;

use protov::formatter::CustomFormatter;
use protov::ProtovDecoder;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Input compiled filedescriptorset (.fdset) file
    #[structopt(short, long, parse(from_os_str))]
    fdset_files: Vec<PathBuf>,

    /// Message type (fully qualified, starts with leading period)
    #[structopt(short, long)]
    message_type: String,
}

use exitfailure::ExitFailure;
use failure::{Error, ResultExt};
use protobuf::parse_from_reader;
use std::fs::File;
use std::path::PathBuf;

pub fn get_loaded_descriptors(fdset_files: Vec<PathBuf>) -> Result<Vec<FileDescriptorSet>, Error> {
    let mut descriptors: Vec<FileDescriptorSet> = Vec::new();

    for fdset_path in fdset_files {
        let mut fdset_file = File::open(fdset_path.as_path())
            .with_context(|_| format!("Couldn't open fdset file: {:?}", fdset_path))?;
        match parse_from_reader(&mut fdset_file) {
            Err(_) => continue,
            Ok(x) => descriptors.push(x),
        }
    }

    if descriptors.is_empty() {
        panic!("No valid fdset files found.");
    }
    Ok(descriptors)
}

fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    let mut stdout = io::stdout();
    let mut printer = Printer::new(&mut stdout, true, BorderStyle::Unicode, true);

    let mut stdin_string = String::new();
    io::stdin()
        .read_to_string(&mut stdin_string)
        .with_context(|_| "Error reading from stdin.")?;
    printer.print_all(Cursor::new(&stdin_string), None).unwrap();

    let bytes: Vec<u8> = stdin_string.bytes().collect();

    let descriptors = get_loaded_descriptors(opt.fdset_files)?;

    let decoder = ProtovDecoder::new(descriptors, &opt.message_type);
    let mut formatter = CustomFormatter::new();

    let v = decoder.decode_message(&bytes)?;
    let mut stdout_ = stdout.lock();
    decoder.write_message(v, &mut stdout_, &mut formatter);
    Ok(())
}
