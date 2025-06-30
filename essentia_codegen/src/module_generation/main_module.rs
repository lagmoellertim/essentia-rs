use std::path::Path;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse_quote;

pub fn generate_main_module_file(
    out_dir: &Path,
    category_module_names: &[String],
) -> std::io::Result<()> {
    let module_file_path = out_dir.join("mod.rs");

    let mut sorted_categorie_module_names = category_module_names.to_vec();
    sorted_categorie_module_names.sort();

    let category_module_declarations: Vec<TokenStream> = sorted_categorie_module_names
        .iter()
        .map(|category_module_name| {
            let category_module_identifier = format_ident!("{}", category_module_name);
            quote! {
                pub mod #category_module_identifier;
            }
        })
        .collect();

    let category_module_re_exports: Vec<TokenStream> = sorted_categorie_module_names
        .iter()
        .map(|category_module_name| {
            let category_module_identifier = format_ident!("{}", category_module_name);
            quote! {
                pub use #category_module_identifier::*;
            }
        })
        .collect();

    let syntax_tree = parse_quote! {
        // Auto-generated file
        // This file contains all category modules

        #(#category_module_declarations)*

        #(#category_module_re_exports)*
    };

    let formatted = prettyplease::unparse(&syntax_tree);
    std::fs::write(module_file_path, formatted)?;
    Ok(())
}
