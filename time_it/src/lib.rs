use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn time_it(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_attrs = &input.attrs;
    let fn_vis = &input.vis;
    let fn_sig = &input.sig;

    let gen = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            let start = std::time::Instant::now();
            let result = (|| #fn_block)();
            let duration = start.elapsed();
            println!("Function `{}` took {:?}", stringify!(#fn_name), duration);
            result
        }
    };

    gen.into()
}
