use essentia_core::DataType;
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
        // If line starts with 4+ spaces, it's likely a code example that shouldn't be tested
        let formatted_line = if line.len() >= 4 && line.chars().take(4).all(|c| c == ' ') {
            // Add a single space to break the 4-space doctest pattern
            format!("  {}", line.trim_start())
        } else {
            format!(" {}", line)
        };
        
        tokens.extend(quote! {
            #[doc = #formatted_line]
        });
    }

    tokens
}

pub fn data_type_to_phantom(data_type: &DataType) -> TokenStream {
    match data_type {
        DataType::Bool => quote! { crate::phantom::Bool },
        DataType::Float => quote! { crate::phantom::Float},
        DataType::String => quote! { crate::phantom::String},
        DataType::Int => quote! { crate::phantom::Int},
        DataType::UnsignedInt => quote! { crate::phantom::UnsignedInt},
        DataType::Long => quote! { crate::phantom::Long},
        DataType::StereoSample => quote! { crate::phantom::StereoSample},
        DataType::Complex => quote! { crate::phantom::Complex},
        DataType::TensorFloat => quote! { crate::phantom::TensorFloat},
        DataType::VectorFloat => quote! { crate::phantom::VectorFloat},
        DataType::VectorString => quote! { crate::phantom::VectorString},
        DataType::VectorBool => quote! { crate::phantom::VectorBool},
        DataType::VectorInt => quote! { crate::phantom::VectorInt},
        DataType::VectorStereoSample => quote! { crate::phantom::VectorStereoSample},
        DataType::VectorComplex => quote! { crate::phantom::VectorComplex},
        DataType::VectorVectorFloat => quote! { crate::phantom::VectorVectorFloat},
        DataType::VectorVectorString => quote! { crate::phantom::VectorVectorString},
        DataType::VectorVectorStereoSample => quote! { crate::phantom::VectorVectorStereoSample},
        DataType::VectorVectorComplex => quote! { crate::phantom::VectorVectorComplex},
        DataType::VectorMatrixFloat => quote! { crate::phantom::VectorMatrixFloat},
        DataType::MapVectorFloat => quote! { crate::phantom::MapVectorFloat},
        DataType::MapVectorString => quote! { crate::phantom::MapVectorString},
        DataType::MapVectorInt => quote! { crate::phantom::MapVectorInt},
        DataType::MapVectorComplex => quote! { crate::phantom::MapVectorComplex},
        DataType::MapFloat => quote! { crate::phantom::MapFloat},
        DataType::MatrixFloat => quote! { crate::phantom::MatrixFloat},
        DataType::Pool => quote! { crate::phantom::Pool},
    }
}
