use essentia_core::variant_data::DataType;
use proc_macro2::TokenStream;
use quote::quote;
use textwrap::fill;

pub fn sanitize_identifier_string(string: &str) -> String {
    match string {
        "type" | "match" | "if" | "else" | "while" | "for" | "loop" | "fn" | "let" | "mut"
        | "const" | "static" | "struct" | "enum" | "impl" | "trait" | "mod" | "pub" | "use"
        | "crate" | "super" | "self" | "Self" | "as" | "where" | "move" | "ref" | "in"
        | "break" | "continue" | "return" | "yield" | "unsafe" | "extern" | "dyn" | "async"
        | "await" | "try" | "macro" | "union" => format!("{}_", string),
        _ => string.to_string(),
    }
}

pub fn string_to_doc_comment(string: &str) -> TokenStream {
    let wrapped = fill(string, 80);
    let lines = wrapped.lines();

    let mut tokens = TokenStream::new();
    for line in lines {
        let line = format!(" {}", line);
        tokens.extend(quote! {
            #[doc = #line]
        });
    }

    tokens
}

pub fn data_type_to_variant(data_type: &DataType) -> TokenStream {
    match data_type {
        DataType::Bool => quote! { variant::Bool },
        DataType::Float => quote! { variant::Float},
        DataType::String => quote! { variant::String},
        DataType::Int => quote! { variant::Int},
        DataType::UnsignedInt => quote! { variant::UnsignedInt},
        DataType::Long => quote! { variant::Long},
        DataType::StereoSample => quote! { variant::StereoSample},
        DataType::Complex => quote! { variant::Complex},
        DataType::TensorFloat => quote! { variant::TensorFloat},
        DataType::VectorFloat => quote! { variant::VectorFloat},
        DataType::VectorString => quote! { variant::VectorString},
        DataType::VectorBool => quote! { variant::VectorBool},
        DataType::VectorInt => quote! { variant::VectorInt},
        DataType::VectorStereoSample => quote! { variant::VectorStereoSample},
        DataType::VectorComplex => quote! { variant::VectorComplex},
        DataType::VectorVectorFloat => quote! { variant::VectorVectorFloat},
        DataType::VectorVectorString => quote! { variant::VectorVectorString},
        DataType::VectorVectorStereoSample => quote! { variant::VectorVectorStereoSample},
        DataType::VectorVectorComplex => quote! { variant::VectorVectorComplex},
        DataType::VectorMatrixFloat => quote! { variant::VectorMatrixFloat},
        DataType::MapVectorFloat => quote! { variant::MapVectorFloat},
        DataType::MapVectorString => quote! { variant::MapVectorString},
        DataType::MapVectorInt => quote! { variant::MapVectorInt},
        DataType::MapVectorComplex => quote! { variant::MapVectorComplex},
        DataType::MapFloat => quote! { variant::MapFloat},
        DataType::MatrixFloat => quote! { variant::MatrixFloat},
        DataType::Pool => quote! { variant::Pool},
    }
}
