use convert_case::{Case, Casing};
use essentia_core::algorithm::AlgorithmIntrospection;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::algorithm_generation::common::{
    data_type_to_variant, sanitize_identifier_string, string_to_doc_comment,
};

fn generate_compute_docs<'a>(algorithm_introspection: &AlgorithmIntrospection) -> TokenStream {
    let mut doc_string_lines = vec!["Computes the algorithm with the given inputs.".to_string()];
    let mut inputs = algorithm_introspection.inputs().peekable();

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
    algorithm_introspection: &AlgorithmIntrospection,
) -> TokenStream {
    let mut p = Vec::new();
    let mut set_statements = Vec::new();

    for input in algorithm_introspection.inputs() {
        let input_name = input.name().to_case(Case::Snake);
        let ident = format_ident!("{}", sanitize_identifier_string(&input_name));
        let variant = data_type_to_variant(&input.input_output_type().into());

        p.push(quote! { #ident: impl TryIntoVariantData<#variant> });
        set_statements.push(quote! { self.algorithm.set_input(#input_name, #ident)?; });
    }

    let doc_comment = generate_compute_docs(algorithm_introspection);

    quote! {
        #doc_comment
        pub fn compute(&mut self, #(#p,)*) -> Result<#algorithm_result_struct_name<'a, '_>, ComputationError> {
            #(#set_statements)*

            Ok(#algorithm_result_struct_name {
                compute_result: self.algorithm.compute()?,
            })
        }
    }
}
