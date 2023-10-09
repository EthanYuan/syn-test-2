pub mod clap_extractor;
pub mod log_extractor;
pub mod std_output_extractor;
pub mod thiserror_extractor;

use crate::types::TextInfo;
use crate::{
    yaml_processor::save_yaml, CLAP_TEXT_FILE, LOG_TEXT_FILE, STD_OUTPUT_TEXT_FILE,
    THISERROR_TEXT_FILE,
};

use cargo_metadata::MetadataCommand;
use syn::visit::visit_file;

use std::{
    fs,
    path::{Path, PathBuf},
};

macro_rules! define_extractor {
    ($struct_name:ident) => {
        #[derive(Default)]
        pub struct $struct_name {
            list: Vec<TextInfo>,
            file_path: PathBuf,
        }

        impl $struct_name {
            pub fn new() -> Self {
                $struct_name::default()
            }

            pub fn reset_analysis_path(&mut self, file_path: &PathBuf) {
                self.file_path = file_path.to_owned();
            }

            pub fn add_text_info(&mut self, text_info: TextInfo) {
                self.list.push(text_info)
            }

            pub fn get_text_list(&self) -> &[TextInfo] {
                &self.list
            }
        }
    };
}

define_extractor!(ClapExtractor);
define_extractor!(LogExtractor);
define_extractor!(StdOutputExtractor);
define_extractor!(ThiserrorExtractor);

pub fn extract(project_root: PathBuf, output_dir: &PathBuf) {
    // extractors
    let mut clap_extractor = ClapExtractor::new();
    let mut log_extractor = LogExtractor::new();
    let mut std_output_extractor = StdOutputExtractor::new();
    let mut thiserror_extractor = ThiserrorExtractor::new();

    let project_metadata = MetadataCommand::new()
        .manifest_path(project_root)
        .exec()
        .expect("Failed to get current directory");

    for package in project_metadata.workspace_packages() {
        println!("Crate Name: {}", package.name);
        println!("Crate Version: {}", package.version);

        let crate_path = Path::new(&package.manifest_path).parent().unwrap();
        process_rs_files_in_src(
            crate_path,
            &mut log_extractor,
            &mut std_output_extractor,
            &mut thiserror_extractor,
            &mut clap_extractor,
        );

        println!();
    }

    save_as_file(
        output_dir,
        clap_extractor,
        log_extractor,
        std_output_extractor,
        thiserror_extractor,
    );
}

pub fn process_rs_files_in_src(
    src_dir: &Path,
    log_extractor: &mut LogExtractor,
    std_output_extractor: &mut StdOutputExtractor,
    thiserror_extractor: &mut ThiserrorExtractor,
    clap_extractor: &mut ClapExtractor,
) {
    if let Ok(entries) = fs::read_dir(&src_dir.join("src")) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".rs") {
                    let file_path = entry.path();
                    println!("Found .rs file: {:?}", file_path);

                    // reset file path
                    clap_extractor.reset_analysis_path(&file_path);
                    log_extractor.reset_analysis_path(&file_path);
                    std_output_extractor.reset_analysis_path(&file_path);
                    thiserror_extractor.reset_analysis_path(&file_path);

                    let file_content = fs::read_to_string(&file_path).expect("Failed to read file");
                    if let Ok(syntax_tree) = syn::parse_file(&file_content) {
                        visit_file(clap_extractor, &syntax_tree);
                        visit_file(log_extractor, &syntax_tree);
                        visit_file(std_output_extractor, &syntax_tree);
                        visit_file(thiserror_extractor, &syntax_tree);
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

fn save_as_file(
    output_dir: &PathBuf,
    clap_extractor: ClapExtractor,
    log_extractor: LogExtractor,
    std_output_extractor: StdOutputExtractor,
    thiserror_extractor: ThiserrorExtractor,
) {
    fs::create_dir_all(output_dir).expect("create dir all");

    save_yaml(
        &output_dir.join(LOG_TEXT_FILE),
        log_extractor.get_text_list(),
    )
    .expect("save yaml");
    save_yaml(
        &output_dir.join(CLAP_TEXT_FILE),
        clap_extractor.get_text_list(),
    )
    .expect("save yaml");
    save_yaml(
        &output_dir.join(STD_OUTPUT_TEXT_FILE),
        std_output_extractor.get_text_list(),
    )
    .expect("save yaml");
    save_yaml(
        &output_dir.join(THISERROR_TEXT_FILE),
        thiserror_extractor.get_text_list(),
    )
    .expect("save yaml");
}
