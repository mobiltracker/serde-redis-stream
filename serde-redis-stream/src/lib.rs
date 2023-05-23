use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(RedisStreamSerialize)]
pub fn redis_stream_serialize(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let DeriveInput { data, ident, .. } = derive_input;

    let fields = match data {
        syn::Data::Struct(s) => s.fields,
        syn::Data::Enum(_) => panic!("Not supported"),
        syn::Data::Union(_) => panic!("Not supported"),
    }
    .into_iter();

    let fields = fields
        .map(|field| {
            let f_ident = field.ident.expect("struct fields must have ident");
            let f_lit = f_ident.to_string();

            quote!(cmd.arg(#f_lit);
                   cmd.arg(&self.#f_ident);)
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        impl serde_redis_stream_interface::RedisStreamSerializable for #ident {
            fn redis_serialize(&self, stream_name: &str, id: &str) -> redis::Cmd {
                let mut cmd: redis::Cmd = redis::cmd("XADD");
                cmd.arg(stream_name).arg(id);
                #(#fields)*
                cmd
            }
        }
    };

    expanded.into()
}
