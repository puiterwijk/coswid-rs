use coswid::CoSWIDTag;

use std::env;
use std::io::Cursor;
use std::fs::File;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: $0 <filename>");
        return Ok(());
    }

    let reader = File::open(&args[1]).unwrap();
    let parsed: CoSWIDTag = ciborium::de::from_reader(&reader)?;

    println!("Parsed CoSWID: {:?}", parsed);

    let mut writer = Vec::new();
    let rebuilt = ciborium::ser::into_writer(&parsed, &mut writer)?;

    println!("Target: {:?}", writer);

    Ok(())
}
