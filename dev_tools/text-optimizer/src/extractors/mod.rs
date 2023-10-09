pub mod clap_extractor;
pub mod log_extractor;
pub mod std_output_extractor;
pub mod thiserror_extractor;

use crate::{yaml_processor::save_yaml, LOG_TEXT_FILE};
use clap_extractor::ClapExtractor;
use log_extractor::LogExtractor;
use std_output_extractor::StdOutputExtractor;
use thiserror_extractor::ThiserrorExtractor;

use cargo_metadata::MetadataCommand;
use syn::visit::visit_file;

use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn extract(project_root: PathBuf, output_dir: &PathBuf) {
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

    fs::create_dir_all(output_dir).expect("create dir all");

    save_yaml(
        &output_dir.join(LOG_TEXT_FILE),
        log_extractor.get_text_list(),
    )
    .expect("save yaml");
}

pub fn process_rs_files_in_src(src_dir: &Path, log_extractor: &mut LogExtractor) {
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

pub fn extract_contents_in_brackets(lit: String) -> Option<String> {
    if let Some(start) = lit.find('"') {
        if let Some(end) = lit.rfind('"') {
            let format_string = &lit[start + 1..end];
            return Some(format_string.to_string());
        }
    }
    None
}
