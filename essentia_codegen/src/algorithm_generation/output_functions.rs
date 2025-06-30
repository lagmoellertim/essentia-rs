use convert_case::{Case, Casing};
use essentia_core::algorithm::{AlgorithmIntrospection, InputOutputInfo};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::algorithm_generation::common::{
    data_type_to_variant, sanitize_identifier_string, string_to_doc_comment,
};

pub fn generate_output_function_docs(output: &InputOutputInfo) -> TokenStream {
    let name = output.name();
    let description = output.description();

    let doc = format!("Returns the `{}` output.\n\n{}", name, description);

    string_to_doc_comment(&doc)
}

pub fn generate_output_functions(
    algorithm_introspection: &AlgorithmIntrospection,
) -> Vec<TokenStream> {
    algorithm_introspection
        .outputs()
        .map(|output| {
            let output_name = output.name();
            let identifier = format_ident!(
                "{}",
                sanitize_identifier_string(&output_name.to_case(Case::Snake))
            );
            let variant = data_type_to_variant(&output.input_output_type().into());
            let doc_comment = generate_output_function_docs(output);

            quote! {
                #doc_comment
                pub fn #identifier(&self) -> Result<VariantData<'result, #variant>, OutputError> {
                    self.compute_result.output::<#variant>(#output_name)
                }
            }
        })
        .collect()
}
