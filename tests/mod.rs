use std::{
    fs::{self, File, read_dir},
    io::BufReader,
    path::Path,
};

use nodescript_rust::tokens::Tokens;

#[test]
fn test_files_validation() {
    for folder in read_dir(Path::new("./data/")).expect("Failed to read data directory") {
        let folder = match folder {
            Ok(folder) if folder.file_type().unwrap().is_dir() => folder.path(),
            _ => continue,
        };
        let script = folder.join("script.ns");
        let input = folder.join("input.in");
        let output = folder.join("output.out");
        let tokens = folder.join("tokens.tok");
        assert!(
            fs::exists(&script).is_ok(),
            "Script file {} does not exist",
            script.display()
        );
        assert!(
            fs::exists(&input).is_ok(),
            "Input file {} does not exist",
            input.display()
        );
        assert!(
            fs::exists(&output).is_ok(),
            "Output file {} does not exist",
            output.display()
        );
        assert!(
            fs::exists(&tokens).is_ok(),
            "Token file {} does not exist",
            tokens.display()
        );
        let file = File::open(&tokens).expect("Failed to open token file");
        let rdr = BufReader::new(file);
        let token: Result<Tokens, serde_json::Error> = serde_json::from_reader(rdr);
        assert!(
            token.is_ok(),
            "Failed to deserialize token file {}",
            tokens.display()
        );
    }
}
