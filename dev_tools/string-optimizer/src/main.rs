mod extractors;

use extractors::{
    clap_extractor::ClapExtractor, log_extractor::LogExtractor,
    std_output_extractor::StdOutputExtractor, thiserror_extractor::ThiserrorExtractor,
};

use cargo_metadata::MetadataCommand;
use std::{fs, path::Path};
use syn::visit::visit_file;

fn main() {
    let project_root = "../../Cargo.toml";

    let metadata = MetadataCommand::new()
        .manifest_path(project_root)
        .exec()
        .expect("Failed to get current directory");

    for package in metadata.workspace_packages() {
        println!("Crate Name: {}", package.name);
        println!("Crate Version: {}", package.version);

        let crate_path = Path::new(&package.manifest_path).parent().unwrap();
        process_rs_files_in_src(crate_path);

        println!();
    }
}

fn process_rs_files_in_src(src_dir: &Path) {
    // Extractors
    let mut std_output_extractor = StdOutputExtractor;
    let mut log_extractor = LogExtractor;
    let mut thiserror_extractor = ThiserrorExtractor;
    let mut clap_extractor = ClapExtractor;

    let src_path = src_dir.join("src");

    if let Ok(entries) = fs::read_dir(&src_path) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".rs") {
                    let file_path = entry.path();
                    println!("Found .rs file: {:?}", file_path);

                    let file_content = fs::read_to_string(&file_path).expect("Failed to read file");

                    if let Ok(syntax_tree) = syn::parse_file(&file_content) {
                        // Traversing the syntax tree
                        visit_file(&mut std_output_extractor, &syntax_tree);
                        visit_file(&mut log_extractor, &syntax_tree);
                        visit_file(&mut thiserror_extractor, &syntax_tree);
                        visit_file(&mut clap_extractor, &syntax_tree);
                    } else {
                        println!("Failed to parse .rs file: {:?}", file_path);
                    }
                }
            }
        }
    }
}
