use std::path::Path;

use convert_case::{Case, Casing};
use essentia_core::algorithm::AlgorithmIntrospection;
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

pub fn generate_algorithm_module(algorithm_introspection: &AlgorithmIntrospection) -> syn::File {
    let algorithm_struct_name = format_ident!(
        "{}",
        &algorithm_introspection.name().trim().to_case(Case::Pascal)
    );

    let algorithm_impl_struct_name = format_ident!(
        "{}Algorithm",
        &algorithm_introspection.name().trim().to_case(Case::Pascal)
    );

    let algorithm_result_struct_name = format_ident!(
        "{}Result",
        &algorithm_introspection.name().trim().to_case(Case::Pascal)
    );

    let algorithm_name = algorithm_introspection.name();
    let description = string_to_doc_comment(algorithm_introspection.description());
    let parameter_functions =
        generate_parameter_functions(algorithm_introspection, &algorithm_impl_struct_name);
    let compute_function = generate_compute_function(
        algorithm_result_struct_name.clone(),
        algorithm_introspection,
    );
    let output_functions = generate_output_functions(algorithm_introspection);

    parse_quote! {
        use essentia_core::{
            algorithm::{Algorithm, Configured, Initialized},
            variant_data::{TryIntoVariantData, VariantData, variant},
        };

        #description
        pub struct #algorithm_impl_struct_name<'a, State> {
            algorithm: Algorithm<'a, State>
        }

        impl <'a> #algorithm_impl_struct_name<'a, Initialized> {
            #(#parameter_functions)*

            /// Configure the algorithm with the set parameters
            ///
            /// Returns a configured algorithm ready for computation.
            pub fn configure(self) -> Result<#algorithm_impl_struct_name<'a, Configured>, crate::error::ConfigureError> {
                Ok(#algorithm_impl_struct_name {
                    algorithm: self.algorithm.configure()?,
                })
            }
        }

        impl <'a> #algorithm_impl_struct_name<'a, Configured> {
            #compute_function
        }

        pub struct #algorithm_struct_name;

        impl<'a> crate::CreateAlgorithm<'a> for #algorithm_struct_name {
            type Output = #algorithm_impl_struct_name<'a, Initialized>;
            fn create(essentia: &'a essentia_core::essentia::Essentia) -> Self::Output {
                let algorithm = essentia
                    .create_algorithm(#algorithm_name)
                    .expect("Algorithm should be available");

                Self::Output { algorithm }
            }
        }

        pub struct #algorithm_result_struct_name<'algorithm, 'result> {
            compute_result: essentia_core::algorithm::ComputeResult<'algorithm, 'result>
        }

        impl <'algorithm, 'result> #algorithm_result_struct_name<'algorithm, 'result> {
            #(#output_functions)*
        }
    }
}

pub fn generate_algorithm_module_file(
    algorithm_introspection: &AlgorithmIntrospection,
    out_dir: &Path,
) -> std::io::Result<GeneratedAlgorithm> {
    let algorithm_module_name = algorithm_introspection.name().trim().to_case(Case::Snake);
    let category_module_name = Regex::new(r"\W+")
        .unwrap()
        .replace_all(algorithm_introspection.category().trim(), " ")
        .trim()
        .to_case(Case::Snake);

    let category_module_directory_path = out_dir.join(&category_module_name);
    let algorithm_module_file_path =
        category_module_directory_path.join(format!("{}.rs", &algorithm_module_name));

    std::fs::create_dir_all(&category_module_directory_path)?;

    let syntax_tree = generate_algorithm_module(algorithm_introspection);
    let formatted = prettyplease::unparse(&syntax_tree);
    std::fs::write(&algorithm_module_file_path, formatted)?;

    Ok(GeneratedAlgorithm {
        algorithm_module_name,
        category_module_name,
    })
}
