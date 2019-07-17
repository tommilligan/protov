use bytes::BytesMut;
use exitfailure::ExitFailure;
use failure::ResultExt;
use prost::Message;
use rand::distributions::{Distribution, Standard};
use rand_core::SeedableRng;
use rand_xorshift::XorShiftRng;
use std::io::{self, Write};
use structopt::StructOpt;

use serialize_emails::Email;

/// Generate a single serialized Email protobuf
#[derive(StructOpt, Debug)]
#[structopt(name = "searialize-emails")]
struct Opt {
    /// Set seed
    #[structopt(short, long, default_value = "0")]
    seed: u64,
}

fn main() -> Result<(), ExitFailure> {
    let opt = Opt::from_args();

    let mut rng = XorShiftRng::seed_from_u64(opt.seed);
    let email: Email = Standard.sample(&mut rng);

    let mut bytes = BytesMut::with_capacity(128);
    email
        .encode(&mut bytes)
        .with_context(|_| "Could not encode email to protobuf.")?;

    io::stdout()
        .write_all(&bytes)
        .with_context(|_| "Could not write to stdout.")?;
    Ok(())
}
