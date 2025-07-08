use convert_case::{Case, Casing};

use essentia_core::Introspection;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::algorithm_generation::common::{
    data_type_enum_to_data_type_marker, sanitize_identifier_string, string_to_doc_comment,
};

fn generate_compute_docs<'a>(introspection: &Introspection) -> TokenStream {
    let mut doc_string_lines = vec!["Computes the algorithm with the given inputs.".to_string()];
    let mut inputs = introspection.inputs().peekable();

    if inputs.peek().is_some() {
        doc_string_lines.push("".to_string());
        doc_string_lines.push("# Inputs".to_string());

        for input in inputs {
            let input_name = sanitize_identifier_string(&input.name().to_case(Case::Snake));
            let description = input.description();
            doc_string_lines.push(format!("* `{}`: {}", input_name, description));
        }
    }

    string_to_doc_comment(&doc_string_lines.join("\n"))
}

pub fn generate_compute_function(
    algorithm_result_struct_name: Ident,
    introspection: &Introspection,
) -> TokenStream {
    let mut p = Vec::new();
    let mut set_statements = Vec::new();

    for input in introspection.inputs() {
        let input_name = input.name().to_case(Case::Snake);
        let ident = format_ident!("{}", sanitize_identifier_string(&input_name));
        let variant = data_type_enum_to_data_type_marker(&input.input_output_type());

        p.push(quote! { #ident: impl crate::data::IntoDataContainer<#variant> });
        set_statements.push(quote! {
            match self.algorithm.set_input(#input_name, #ident) {
                Ok(_) => {},
                Err(essentia_core::algorithm::InputError::InputNotFound { input }) => {
                    panic!("Input '{}' not found after validation", input);
                }
                Err(essentia_core::algorithm::InputError::TypeMismatch { input, expected, actual }) => {
                    panic!("Type mismatch for input '{}': expected {:?}, found {:?}", input, expected, actual);
                }
            }
        });
    }

    let doc_comment = generate_compute_docs(introspection);

    quote! {
        #doc_comment
        pub fn compute(&mut self, #(#p,)*) -> Result<#algorithm_result_struct_name<'a, '_>, crate::algorithm::ComputeError> {
            #(#set_statements)*

            Ok(#algorithm_result_struct_name {
                compute_result: self.algorithm.compute().map_err(|e| match e {
                    essentia_core::algorithm::ComputeError::Compute(exception) => {
                        crate::algorithm::ComputeError::Compute(exception)
                    }
                })?,
            })
        }
    }
}
