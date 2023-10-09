use super::extract_contents_in_brackets;
use syn::Expr::{self};
use syn::Lit::Str;

pub struct ClapExtractor;

impl syn::visit::Visit<'_> for ClapExtractor {
    fn visit_expr_method_call(&mut self, expr: &syn::ExprMethodCall) {
        let method_ident = &expr.method;
        if *method_ident == "about" || *method_ident == "help" {
            if let Some(Expr::Lit(lit)) = expr.args.first() {
                if let Str(lit_str) = &lit.lit {
                    let lit = lit_str.token().to_string();

                    let format_string = extract_contents_in_brackets(lit);
                    if let Some(format_string) = format_string {
                        println!("Found format string: {}", format_string);
                    }
                }
            }
        }
        syn::visit::visit_expr_method_call(self, expr);
    }
}
