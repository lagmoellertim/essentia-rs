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

pub fn generate_algorithm_module(introspection: &Introspection) -> syn::File {
    let algorithm_struct_name =
        format_ident!("{}", &introspection.name().trim().to_case(Case::Pascal));

    let algorithm_result_struct_name = format_ident!(
        "{}Result",
        &introspection.name().trim().to_case(Case::Pascal)
    );

    let algorithm_name = introspection.name();
    let description = string_to_doc_comment(introspection.description());
    let parameter_result = generate_parameter_functions(introspection);
    let compute_function =
        generate_compute_function(algorithm_result_struct_name.clone(), introspection);
    let output_functions = generate_output_functions(introspection);

    let constraint_code = &parameter_result.constraint_code;
    let parameter_functions = &parameter_result.functions;

    parse_quote! {
        #constraint_code

        #description
        #[allow(dead_code)]
        pub struct #algorithm_struct_name<'a, State = crate::Initialized> {
            algorithm: essentia_core::algorithm::Algorithm<'a, State>
        }

        impl <'a> #algorithm_struct_name<'a, crate::Initialized> {
            #(#parameter_functions)*

            /// Configure the algorithm with the set parameters
            ///
            /// Returns a configured algorithm ready for computation.
            pub fn configure(self) -> Result<#algorithm_struct_name<'a, crate::Configured>, crate::algorithm::ConfigurationError> {
                Ok(#algorithm_struct_name {
                    algorithm: self.algorithm.configure().map_err(|e| match e {
                        essentia_core::algorithm::ConfigurationError::Internal(exception) => {
                            crate::algorithm::ConfigurationError::Internal(exception)
                        }
                    })?,
                })
            }
        }

        impl <'a> #algorithm_struct_name<'a, crate::Configured> {
            #compute_function
        }

        impl<'a> crate::algorithm::CreateAlgorithm<'a> for #algorithm_struct_name<'a, crate::Initialized> {
            fn create(essentia: &'a crate::Essentia) -> Self {
                let algorithm = match essentia.inner.create_algorithm(#algorithm_name) {
                    Ok(algorithm) => algorithm,
                    Err(essentia_core::CreateAlgorithmError::AlgorithmNotFound { name }) => {
                        panic!("Algorithm '{}' not found in Essentia", name);
                    }
                };

                Self { algorithm }
            }
        }

        #[allow(dead_code)]
        pub struct #algorithm_result_struct_name<'algorithm, 'result> {
            compute_result: essentia_core::algorithm::ComputeResult<'algorithm, 'result>
        }

        impl <'algorithm, 'result> #algorithm_result_struct_name<'algorithm, 'result> {
            #(#output_functions)*
        }
    }
}

pub fn generate_algorithm_module_file(
    introspection: &Introspection,
    out_dir: &Path,
) -> std::io::Result<GeneratedAlgorithm> {
    let algorithm_module_name = introspection.name().trim().to_case(Case::Snake);
    let category_module_name = Regex::new(r"\W+")
        .unwrap()
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
