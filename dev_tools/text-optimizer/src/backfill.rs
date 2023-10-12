use crate::yaml_processor::load_yaml;

use std::fs::read_dir;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn backfill(input_dir: &PathBuf) {
    if let Ok(entries) = read_dir(input_dir) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with("log_text_list.yml") {
                    log::trace!("{:?}", file_name);
                    let log_text_list = load_yaml(&entry_path).expect("load yaml");
                    for text in log_text_list {
                        let mut source_code = String::new();
                        let mut file =
                            File::open(text.metadata().file()).expect("Failed to open file");
                        file.read_to_string(&mut source_code)
                            .expect("Failed to read file");

                        // Replace the match with the new string
                        let new_source_code = source_code.replace(text.original(), text.editable());

                        // Reopen the file in write mode and write the new source code
                        let mut new_file =
                            File::create(&text.metadata().file()).expect("Failed to create file");
                        new_file
                            .write_all(new_source_code.as_bytes())
                            .expect("Failed to write file");

                        break;
                    }
                    break;
                }
            }
        }
    }
}
