use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(RedisStreamSerialize, attributes(serialize))]
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
            let attrs = field.attrs.iter().find(|a| match &a.meta {
                syn::Meta::NameValue(nv) => {
                    &nv.path.get_ident().unwrap().to_string() == "serialize"
                }
                _ => false,
            });

            if let Some(serialize) = attrs {
                let serialization_method = match &serialize.meta {
                    syn::Meta::NameValue(nv) => match &nv.value {
                        syn::Expr::Lit(lit) => match &lit.lit {
                            syn::Lit::Str(lit) => lit.value(),
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                };

                match serialization_method.as_str() {
                    "bincode" => {
                        quote!(
                            let encoded: Vec<u8> = bincode::serialize(&self.#f_ident).expect("Failed to serialize to bincode");
                            cmd.arg(#f_lit);
                            cmd.arg(encoded);
                        )
                    }
                    _ => panic!("Invalid serialization method {}", serialization_method),
                }
            } else {
                quote!(cmd.arg(#f_lit);
                cmd.arg(&self.#f_ident);)
            }
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
