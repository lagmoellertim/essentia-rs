use std::path::Path;

use convert_case::{Case, Casing};
use essentia_core::Introspection;
use quote::format_ident;
use regex::Regex;
use syn::parse_quote;

use crate::algorithm_generation::{
    common::string_to_doc_comment, compute_function::generate_compute_function,
    output_functions::generate_output_functions, parameter_functions::generate_parameter_functions,
};

mod common;
mod compute_function;
mod output_functions;
mod parameter_functions;

pub struct GeneratedAlgorithm {
    pub algorithm_module_name: String,
    pub category_module_name: String,
}

/// Creates the struct name identifiers for the algorithm
fn create_algorithm_identifiers(introspection: &Introspection) -> (syn::Ident, syn::Ident) {
    let algorithm_struct_name =
        format_ident!("{}", &introspection.name().trim().to_case(Case::Pascal));

    let algorithm_result_struct_name = format_ident!(
        "{}Result",
        &introspection.name().trim().to_case(Case::Pascal)
    );

    (algorithm_struct_name, algorithm_result_struct_name)
}

/// Generates the main algorithm struct and its implementations
fn generate_algorithm_struct(
    algorithm_struct_name: syn::Ident,
    algorithm_result_struct_name: syn::Ident,
    introspection: &Introspection,
) -> syn::ItemStruct {
    let description = string_to_doc_comment(introspection.description());
    
    parse_quote! {
        #description
        #[allow(dead_code)]
        pub struct #algorithm_struct_name<'a, State = crate::Initialized> {
            algorithm: crate::algorithm::dynamic::Algorithm<'a, State>
        }
    }
}

/// Generates the implementation blocks for the algorithm struct
fn generate_algorithm_implementations(
    algorithm_struct_name: &syn::Ident,
    algorithm_result_struct_name: &syn::Ident,
    introspection: &Introspection,
) -> Vec<syn::ItemImpl> {
    let algorithm_name = introspection.name();
    let parameter_functions = generate_parameter_functions(introspection);
    let compute_function =
        generate_compute_function(algorithm_result_struct_name.clone(), introspection);

    vec![
        parse_quote! {
            impl <'a> #algorithm_struct_name<'a, crate::Initialized> {
                #(#parameter_functions)*

                /// Configure the algorithm with the set parameters
                ///
                /// Returns a configured algorithm ready for computation.
                pub fn configure(self) -> Result<#algorithm_struct_name<'a, crate::Configured>, crate::algorithm::ConfigurationError> {
                    Ok(#algorithm_struct_name {
                        algorithm: self.algorithm.configure()?,
                    })
                }
            }
        },
        parse_quote! {
            impl <'a> #algorithm_struct_name<'a, crate::Configured> {
                #compute_function
            }
        },
        parse_quote! {
            impl<'a> crate::algorithm::CreateAlgorithm<'a> for #algorithm_struct_name<'a, crate::Initialized> {
                type Output = #algorithm_struct_name<'a, crate::Initialized>;
                fn create(essentia: &'a crate::Essentia) -> Self::Output {
                    let algorithm = essentia
                        .create_from_name(#algorithm_name)
                        .expect("Algorithm should be available");

                    Self::Output { algorithm }
                }
            }
        }
    ]
}

/// Generates the result struct and its implementation
fn generate_result_struct(
    algorithm_result_struct_name: syn::Ident,
    introspection: &Introspection,
) -> (syn::ItemStruct, syn::ItemImpl) {
    let output_functions = generate_output_functions(introspection);

    let struct_def = parse_quote! {
        #[allow(dead_code)]
        pub struct #algorithm_result_struct_name<'algorithm, 'result> {
            compute_result: crate::algorithm::dynamic::ComputeResult<'algorithm, 'result>
        }
    };

    let impl_block = parse_quote! {
        impl <'algorithm, 'result> #algorithm_result_struct_name<'algorithm, 'result> {
            #(#output_functions)*
        }
    };

    (struct_def, impl_block)
}

pub fn generate_algorithm_module(introspection: &Introspection) -> syn::File {
    let (algorithm_struct_name, algorithm_result_struct_name) = 
        create_algorithm_identifiers(introspection);

    let main_struct = generate_algorithm_struct(
        algorithm_struct_name.clone(),
        algorithm_result_struct_name.clone(),
        introspection,
    );

    let impl_blocks = generate_algorithm_implementations(
        &algorithm_struct_name,
        &algorithm_result_struct_name,
        introspection,
    );

    let (result_struct, result_impl) = generate_result_struct(
        algorithm_result_struct_name,
        introspection,
    );

    // Combine all items into a single file
    let mut items = Vec::new();
    items.push(syn::Item::Struct(main_struct));
    for impl_block in impl_blocks {
        items.push(syn::Item::Impl(impl_block));
    }
    items.push(syn::Item::Struct(result_struct));
    items.push(syn::Item::Impl(result_impl));

    syn::File {
        shebang: None,
        attrs: vec![],
        items,
    }
}

pub fn generate_algorithm_module_file(
    introspection: &Introspection,
    out_dir: &Path,
) -> std::io::Result<GeneratedAlgorithm> {
    let algorithm_module_name = introspection.name().trim().to_case(Case::Snake);
    
    // Use a more robust way to create regex - this should never fail for our simple pattern
    let category_regex = Regex::new(r"\W+")
        .expect("Failed to compile regex pattern for category normalization");
    let category_module_name = category_regex
        .replace_all(introspection.category().trim(), " ")
        .trim()
        .to_case(Case::Snake);

    let category_module_directory_path = out_dir.join(&category_module_name);
    let algorithm_module_file_path =
        category_module_directory_path.join(format!("{}.rs", &algorithm_module_name));

    std::fs::create_dir_all(&category_module_directory_path)?;

    let syntax_tree = generate_algorithm_module(introspection);
    let formatted = prettyplease::unparse(&syntax_tree);
    std::fs::write(&algorithm_module_file_path, formatted)?;

    Ok(GeneratedAlgorithm {
        algorithm_module_name,
        category_module_name,
    })
}
