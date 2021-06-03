extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(StringParser)]
pub fn parse_string_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        use pest_string::expr;
        use pest_string::parser;
        use pest_string::scanner;

        impl StringParser for #name {
            fn parse_string(input: String) -> Result<Vec<expr::Stmt>, Error> {
                let tokens = scanner::scan_tokens(input).unwrap();
                parser::parse(tokens)
            }
        }
    };
    gen.into()
}
