use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use schemars::schema_for;

fn main() {
    //     let args: Vec<String> = env::args().collect();
    //     let output_path = if args.len() > 1 {
    //         args[1].clone()
    //     } else {
    //         let mut crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    //         crate_root.push("config_schema.json");
    //         crate_root.to_str().unwrap().to_string()
    //     };

    //     let schema = schema_for!(edlint_config::ConfigSchema);
    //     let json = serde_json::to_string_pretty(&schema).expect("Failed to serialize JSON schema");

    //     let mut f = File::create(&output_path).expect("Failed to create file");
    //     f.write_all(json.as_bytes()).expect("Failed to write file");

    //     println!("JSON schema generated in {}", output_path);
}
