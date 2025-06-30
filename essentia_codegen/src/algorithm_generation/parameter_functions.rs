use convert_case::{Case, Casing};
use essentia_core::algorithm::{AlgorithmIntrospection, ParameterInfo};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn;

use crate::algorithm_generation::common::{data_type_to_variant, sanitize_identifier_string, string_to_doc_comment};



fn generate_parameter_function_docs(parameter: &ParameterInfo) -> TokenStream {
    let name = parameter.name();
    let description = parameter.description();
    let doc = format!(
        "Sets the `{}` parameter.\n\n{}",
        name, description
    );
    
    string_to_doc_comment(&doc)
}

pub fn generate_parameter_functions(
    algorithm_introspection: &AlgorithmIntrospection,
    struct_name: &syn::Ident,
) -> Vec<TokenStream> {
    algorithm_introspection
        .parameters()
        .map(|parameter| {
            let parameter_name = parameter.name();
            let identifier = format_ident!("{}", sanitize_identifier_string(&parameter_name.to_case(Case::Snake)));
            let variant = data_type_to_variant(&parameter.parameter_type().into());
            let doc_comment = generate_parameter_function_docs(parameter);

            quote! {                
                #doc_comment
                pub fn #identifier(mut self, value: impl TryIntoVariantData<#variant>) -> Result<Self, essentia_core::algorithm::ParameterError> {
                    self.algorithm.set_parameter(#parameter_name, value)?;
                    Ok(self)
                }
            }
        })
        .collect()
}