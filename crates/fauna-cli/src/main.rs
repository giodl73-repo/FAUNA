use fauna_core::{has_errors, validate_document, FaunaDocument};
use std::{env, error::Error, fs, io};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();
    match args.as_slice() {
        [_, command, path] if command == "validate" => {
            let text = fs::read_to_string(path)?;
            let document = serde_json::from_str::<FaunaDocument>(&text)?;
            let findings = validate_document(&document);
            println!("{}", serde_json::to_string_pretty(&findings)?);
            if has_errors(&findings) {
                std::process::exit(1);
            }
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "usage: fauna validate <fauna-document.json>",
            )
            .into());
        }
    }
    Ok(())
}
