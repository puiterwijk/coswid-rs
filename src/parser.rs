use coswid::{CoSWIDTag, Result};

use std::env;
use std::fs::File;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: $0 <filename>");
        return Ok(());
    }

    let reader = File::open(&args[1]).unwrap();
    let parsed: CoSWIDTag = ciborium::de::from_reader(&reader)?;

    println!("Parsed CoSWID: {:?}", parsed);

    Ok(())
}
