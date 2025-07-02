use convert_case::{Case, Casing};
use essentia_core::Introspection;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::algorithm_generation::common::{
    data_type_to_phantom, sanitize_identifier_string, string_to_doc_comment,
};

pub fn generate_output_functions(introspection: &Introspection) -> Vec<TokenStream> {
    introspection
        .outputs()
        .map(|output| {
            let method_name = format_ident!(
                "{}",
                &sanitize_identifier_string(&output.name().to_case(Case::Snake))
            );
            let output_name = output.name();
            let variant = data_type_to_phantom(&output.input_output_type().into());
            
            let doc_comment = string_to_doc_comment(&format!(
                "Get the `{}` output from the computation result.\n\n# Description\n\n{}",
                output_name,
                output.description()
            ));

            quote! {
                #doc_comment
                pub fn #method_name(&self) -> Result<crate::DataContainer<'result, #variant>, crate::algorithm::OutputError> {
                    Ok(self.compute_result.output(#output_name)?)
                }
            }
        })
        .collect()
}
