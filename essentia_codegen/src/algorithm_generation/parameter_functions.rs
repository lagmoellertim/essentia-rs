use convert_case::{Case, Casing};
use essentia_core::{algorithm::{Constraint, ParameterInfo}, DataType, Introspection};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::algorithm_generation::common::{
    data_type_enum_to_data_type_marker, sanitize_identifier_string, string_to_doc_comment,
};

fn generate_parameter_function_docs(parameter: &ParameterInfo) -> TokenStream {
    let name = parameter.name();
    let description = parameter.description();
    let doc = format!("Sets the `{}` parameter.\n\n{}", name, description);

    string_to_doc_comment(&doc)
}

pub struct ConstraintInfo {
    trait_ident: syn::Ident,
    constraint_code: TokenStream,
}

fn generate_string_enum_constraint(algorithm_name: &str, parameter_name: &str, options: &[String]) -> ConstraintInfo {
    let algorithm_pascal = algorithm_name.trim().to_case(Case::Pascal);
    let parameter_pascal = parameter_name.to_case(Case::Pascal);
    let constraint_trait_ident = format_ident!("{}{}Constraint", algorithm_pascal, parameter_pascal);
    let enum_ident = format_ident!("{}{}", algorithm_pascal, parameter_pascal);
    
    let enum_variants: Vec<_> = options
        .iter()
        .map(|option| format_ident!("{}", option.to_case(Case::Pascal)))
        .collect();
    
    let string_conversions: Vec<_> = options
        .iter()
        .zip(&enum_variants)
        .map(|(original_name, variant_ident)| {
            quote! { #enum_ident::#variant_ident => #original_name }
        })
        .collect();
    
    let constraint_code = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum #enum_ident {
            #(#enum_variants,)*
        }
        
        impl crate::data::IntoDataContainer<crate::data_type::String> for #enum_ident {
            fn into_data_container(self) -> crate::data::DataContainer<'static, crate::data_type::String> {
                let string_value = match self {
                    #(#string_conversions,)*
                };
                string_value.into_data_container()
            }
        }
        
        pub trait #constraint_trait_ident {}
        
        impl #constraint_trait_ident for #enum_ident {}
    };
    
    ConstraintInfo {
        trait_ident: constraint_trait_ident,
        constraint_code,
    }
}

pub fn generate_constraint(algorithm_name: &str, parameter: &ParameterInfo) -> Option<ConstraintInfo> {
    match (parameter.parameter_type(), parameter.constraint()) {
        (DataType::String, Constraint::OneOf(options)) => {
            Some(generate_string_enum_constraint(algorithm_name, parameter.name(), options))
        }
        _ => None,
    }
}

pub struct ParameterFunctionResult {
    pub functions: Vec<TokenStream>,
    pub constraint_code: TokenStream,
}

pub fn generate_parameter_functions(algorithm_introspection: &Introspection) -> ParameterFunctionResult {
    let mut constraint_code_blocks = Vec::new();
    let algorithm_name = algorithm_introspection.name();
    
    let parameter_functions: Vec<TokenStream> = algorithm_introspection
        .parameters()
        .map(|parameter| {
            let parameter_name = parameter.name();
            let function_name = format_ident!("{}", sanitize_identifier_string(&parameter_name.to_case(Case::Snake)));
            let data_type_variant = data_type_enum_to_data_type_marker(&parameter.parameter_type());
            let doc_comment = generate_parameter_function_docs(parameter);
            
            let type_constraint = match generate_constraint(algorithm_name, parameter) {
                Some(constraint_info) => {
                    constraint_code_blocks.push(constraint_info.constraint_code);
                    let trait_ident = constraint_info.trait_ident;
                    quote! { crate::data::IntoDataContainer<#data_type_variant> + #trait_ident }
                }
                None => {
                    quote! { crate::data::IntoDataContainer<#data_type_variant> }
                }
            };

            quote! {
                #doc_comment
                pub fn #function_name<T>(mut self, value: T) -> Self 
                where 
                    T: #type_constraint 
                {
                    match self.algorithm.set_parameter(#parameter_name, value) {
                        Ok(_) => {},
                        Err(essentia_core::algorithm::ParameterError::ParameterNotFound { parameter }) => {
                            panic!("Parameter '{}' not found after validation", parameter);
                        }
                        Err(essentia_core::algorithm::ParameterError::TypeMismatch { parameter, expected, actual }) => {
                            panic!("Type mismatch for parameter '{}': expected {:?}, found {:?}", parameter, expected, actual);
                        }
                    }
                    self
                }
            }
        })
        .collect();
    
    let constraint_code = quote! { 
        #(#constraint_code_blocks)* 
    };
    
    ParameterFunctionResult {
        functions: parameter_functions,
        constraint_code,
    }
}
