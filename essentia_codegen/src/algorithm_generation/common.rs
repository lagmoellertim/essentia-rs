use essentia_core::DataType;
use proc_macro2::TokenStream;
use quote::quote;
use textwrap::fill;

/// Sanitizes identifier strings to avoid Rust keyword conflicts.
/// 
/// If the input string is a Rust keyword, appends an underscore to make it a valid identifier.
/// This ensures generated code compiles even when algorithm names or parameters match Rust keywords.
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

/// Converts a string into a sequence of doc comment tokens.
/// 
/// Wraps the text to 80 characters and generates appropriate `#[doc = "..."]` attributes
/// for use in generated Rust code.
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

/// Maps a DataType to its corresponding phantom type token stream.
/// 
/// This function provides the mapping between runtime data types and their
/// compile-time phantom type representations used for type safety in the generated code.
pub fn data_type_to_phantom(data_type: &DataType) -> TokenStream {
    macro_rules! phantom_match {
        ($($variant:ident),+ $(,)?) => {
            match data_type {
                $(DataType::$variant => quote! { crate::phantom::$variant },)+
            }
        };
    }

    phantom_match! {
        Bool,
        Float,
        String,
        Int,
        UnsignedInt,
        Long,
        StereoSample,
        Complex,
        TensorFloat,
        VectorFloat,
        VectorString,
        VectorBool,
        VectorInt,
        VectorStereoSample,
        VectorComplex,
        VectorVectorFloat,
        VectorVectorString,
        VectorVectorStereoSample,
        VectorVectorComplex,
        VectorMatrixFloat,
        MapVectorFloat,
        MapVectorString,
        MapVectorInt,
        MapVectorComplex,
        MapFloat,
        MatrixFloat,
        Pool,
    }
}
