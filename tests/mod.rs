use nodescript_rust::*;
use std::{
    fs::{File, read_dir, read_to_string},
    io::BufReader,
    path::{Path, PathBuf},
};

#[test]
fn basic_parse() {
    let data = read_dir(Path::new("./data/")).expect("Failed to read data directory");
    let mut test_cases: Vec<PathBuf> = vec![];
    for folder in data {
        match folder {
            Ok(folder) if folder.file_type().unwrap().is_dir() => {
                test_cases.push(folder.path());
            }
            _ => continue,
        }
    }

    for base_path in test_cases {
        let script = base_path.join("script.ns");
        let result = match tokenize_string(read_to_string(&script).expect("Failed to read file")) {
            Ok(tokens) => tokens,
            Err(e) => panic!("Failed to tokenize: {}", e),
        };

        let cmp_path = base_path.join("tokens.tok");
        let file = File::open(&cmp_path).expect("Failed to open token file");
        let reader = BufReader::new(file);
        let cmp: tokens::Tokens =
            serde_json::from_reader(reader).expect("Failed to deserialize token file");
        println!("Successfully tokenized file {}", script.display());
        assert_eq!(result, cmp)
    }
}
