use super::extract_contents_in_brackets;
use crate::types::{Category, Meta, TextInfo};
use std::path::PathBuf;
use std::str::FromStr;
use syn::spanned::Spanned;
use syn::Macro;

#[derive(Default)]
pub struct LogExtractor {
    list: Vec<TextInfo>,
    file_path: PathBuf,
}

impl LogExtractor {
    pub fn new() -> Self {
        LogExtractor::default()
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

impl syn::visit::Visit<'_> for LogExtractor {
    fn visit_macro(&mut self, node: &Macro) {
        if let Some(ident) = node.path.get_ident() {
            // Determine if the macro is println!
            if ident == "error"
                || ident == "warn"
                || ident == "info"
                || ident == "debug"
                || ident == "trace"
            {
                let lit = node.tokens.to_string();

                if let Some(text) = extract_contents_in_brackets(lit) {
                    println!("Found format string: {}", text);

                    let span = node.span();
                    let start_line = span.start().line;
                    let end_line = span.end().line;
                    let category = Category::from_str(ident.to_string().as_str()).unwrap();
                    let meta = Meta::new(category, self.file_path.to_owned(), start_line, end_line);
                    self.add_text_info(TextInfo::new(text, meta));
                }
            }
        }
    }
}
