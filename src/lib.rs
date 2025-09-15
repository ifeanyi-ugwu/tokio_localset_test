use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn localset_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let block = &input.block;

    // Expand to a tokio test wrapped in LocalSet
    let expanded = quote! {
        #[tokio::test(flavor = "current_thread")]
        async fn #name() {
            let local = tokio::task::LocalSet::new();
            local.run_until(async #block).await;
        }
    };

    expanded.into()
}
