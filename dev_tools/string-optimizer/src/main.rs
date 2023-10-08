mod extractors;
mod types;
mod yaml_processor;

use cargo_metadata::MetadataCommand;
use extractors::{
    clap_extractor::ClapExtractor, log_extractor::LogExtractor,
    std_output_extractor::StdOutputExtractor, thiserror_extractor::ThiserrorExtractor,
};
use std::{fs, path::Path};
use syn::visit::visit_file;
use yaml_processor::save_yaml;

fn main() {
    // root
    let project_root = "../../Cargo.toml";

    // extractors
    let mut log_extractor = LogExtractor::new();

    let project_metadata = MetadataCommand::new()
        .manifest_path(project_root)
        .exec()
        .expect("Failed to get current directory");

    for package in project_metadata.workspace_packages() {
        println!("Crate Name: {}", package.name);
        println!("Crate Version: {}", package.version);

        let crate_path = Path::new(&package.manifest_path).parent().unwrap();
        process_rs_files_in_src(crate_path, &mut log_extractor);

        println!();
    }

    save_yaml("log_text_list.yml", log_extractor.get_text_list()).expect("save yaml");
}

fn process_rs_files_in_src(src_dir: &Path, log_extractor: &mut LogExtractor) {
    if let Ok(entries) = fs::read_dir(&src_dir.join("src")) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".rs") {
                    let file_path = entry.path();
                    println!("Found .rs file: {:?}", file_path);

                    // reset file path
                    log_extractor.reset_analysis_path(&file_path);

                    let file_content = fs::read_to_string(&file_path).expect("Failed to read file");
                    if let Ok(syntax_tree) = syn::parse_file(&file_content) {
                        let mut std_output_extractor = StdOutputExtractor;
                        visit_file(&mut std_output_extractor, &syntax_tree);

                        visit_file(log_extractor, &syntax_tree);

                        let mut thiserror_extractor = ThiserrorExtractor;
                        visit_file(&mut thiserror_extractor, &syntax_tree);

                        let mut clap_extractor = ClapExtractor;
                        visit_file(&mut clap_extractor, &syntax_tree);
                    } else {
                        println!("Failed to parse .rs file: {:?}", file_path);
                    }
                }
            }
        }
    }
}
