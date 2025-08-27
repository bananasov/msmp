use msmp_generator::generate_and_write_types;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!(
            "Usage: {} <rpc-discovery-json-file> <types-crate-path>",
            args[0]
        );
        return Err("Invalid arguments".into());
    }

    let json_file = &args[1];
    let types_path = PathBuf::from(&args[2]);

    println!("Reading RPC discovery response from˸ {}", json_file);
    let data = fs::read_to_string(json_file)?;

    println!("Generating types ᐧᐧᐧ");
    generate_and_write_types(&data, &types_path)?;

    println!("Types generated successfully ඞ");

    Ok(())
}
