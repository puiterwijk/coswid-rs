use coswid::CoSWIDTag;

use std::env;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: $0 <filename> [raw]");
        return Ok(());
    }

    let reader = File::open(&args[1]).unwrap();

    if args.len() == 3 && args[2] == "raw" {
        let raw_parsed: ciborium::value::Value = ciborium::de::from_reader(&reader)?;
        println!("Raw parsed: {:?}", raw_parsed);
        return Ok(());
    }
    let parsed: CoSWIDTag = ciborium::de::from_reader(&reader)?;
    println!("Parsed CoSWID: {:?}", parsed);

    let mut rebuilt = Vec::new();
    ciborium::ser::into_writer(&parsed, &mut rebuilt)?;
    println!("Rebuilt cbor: {:?}", rebuilt);

    let rebuilt = serde_json::to_string(&parsed)?;
    println!("Rebuilt JSON: {:?}", rebuilt);

    Ok(())
}
