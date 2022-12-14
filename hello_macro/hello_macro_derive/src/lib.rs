extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    //construct a representation of Rust code as a sythax tree
    //That we can manipulate
    let ast = syn::parse(input).unwrap();
    //Build the trait implementation
    impl__hello_macro(&ast)
}
fn impl__hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.dent;
    let gen = quote! {
        impl HelloMacro for #name{
            fn hello_macro(){
                println!("Hello, Macro: My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
