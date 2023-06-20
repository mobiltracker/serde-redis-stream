use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(RedisStreamSerialize, attributes(serialize))]
pub fn redis_stream_serialize(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let DeriveInput { data, ident, .. } = derive_input;

    let fields = match data.clone() {
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
                            let encoded: Vec<u8> = bincode::serialize(&self.#f_ident).map_err(|_| serde_redis_stream_interface::RedisStreamDeriveError::SerializationErrorToBincode(String::from(#f_lit)))?;
                            cmd.arg(#f_lit);
                            cmd.arg(encoded);
                        )
                    }
                    "json" => {
                        quote!(
                            let encoded = serde_json::to_string(&self.#f_ident).map_err(|_| serde_redis_stream_interface::RedisStreamDeriveError::SerializationErrorToJSON(String::from(#f_lit)))?;
                            cmd.arg(#f_lit);
                            cmd.arg(encoded);
                        )
                    }
                    _ => panic!("Invalid serialization method {}", serialization_method),
                }
            } else {
                quote!(
                cmd.arg(#f_lit);
                cmd.arg(&self.#f_ident);
            )
            }
        })
        .collect::<Vec<_>>();

    let fields_for_serialization = match data.clone() {
        syn::Data::Struct(s) => s.fields,
        syn::Data::Enum(_) => panic!("Not supported"),
        syn::Data::Union(_) => panic!("Not supported"),
    }
    .into_iter();
    
    let fields_for_serialization = fields_for_serialization
        .map(|field| {
            let f_ident = field.ident.expect("struct fields must have ident");
            let f_lit = f_ident.to_string();
            let (f_type,is_option) = match field.ty{
                syn::Type::Path(not_type) => {                   
                    let segments_str = &not_type.path.segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect::<Vec<_>>()
                    .join(":");                 

                    let is_option = ["Option", "std:option:Option", "core:option:Option"]
                    .iter()
                    .any(|s| segments_str == s); 

                    let temp;

                    if is_option {
                        let option_segment = ["Option", "std:option:Option", "core:option:Option"]
                        .iter()
                        .find(|s| segments_str == *s)
                        .and_then(|_| not_type.path.segments.last());

                        let inner_type = option_segment           
                        .and_then(|path_seg| match &path_seg.arguments {
                            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                                args,
                                ..
                            }) => args.first(),
                            _ => None,
                        })          
                        .and_then(|generic_arg| match generic_arg {
                            syn::GenericArgument::Type(ty) => Some(ty),
                            _ => None,
                        });

                        temp = match inner_type{
                            Some(syn::Type::Path(not_type)) => not_type.path.get_ident().map(|value| value.to_owned()).unwrap(),
                            _ => unimplemented!(),
                        }
                    }else{
                        temp = not_type.path.get_ident().map(|value| value.to_owned()).unwrap();
                    }

                    (temp,is_option)
                },
                _ => unimplemented!(),
            };      

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
                match (serialization_method.as_str(),is_option) {
                    ("bincode",false) => {
                        quote!(
                            let #f_ident: &redis::Value = map.get(#f_lit).ok_or_else(|| serde_redis_stream_interface::RedisStreamDeriveError::MissingFieldFromHashMap(String::from(#f_lit)))?;
                            let #f_ident = <Vec<u8> as redis::FromRedisValue>::from_redis_value(#f_ident).map_err(|_| serde_redis_stream_interface::RedisStreamDeriveError::MissingFieldFromHashMap(String::from(#f_lit)))?;              
                            let #f_ident = bincode::deserialize(&#f_ident).map_err(|_| serde_redis_stream_interface::RedisStreamDeriveError::DeserializationErrorFromBincode(String::from(#f_lit)))?;
                        )
                    }
                    ("bincode",true) => {
                        quote!(
                            let #f_ident: &redis::Value = map.get(#f_lit).ok();
                            let #f_ident  = match #f_ident {
                                Some(#f_ident) => {
                                    let #f_ident = <Vec<u8> as redis::FromRedisValue>::from_redis_value(#f_ident).map_err(|_| serde_redis_stream_interface::RedisStreamDeriveError::MissingFieldFromHashMap(String::from(#f_lit)))?;              
                                    bincode::deserialize(&#f_ident).map_err(|_| serde_redis_stream_interface::RedisStreamDeriveError::DeserializationErrorFromBincode(String::from(#f_lit)))?
                                }
                                None => {
                                    None
                                }
                            };                 
                        )
                    }
                    ("json",false) => {
                        quote!(
                            let #f_ident: &redis::Value = map.get(#f_lit).ok_or_else(|| serde_redis_stream_interface::RedisStreamDeriveError::MissingFieldFromHashMap(String::from(#f_lit)))?;
                            let #f_ident = <String as redis::FromRedisValue>::from_redis_value(#f_ident).map_err(|_| serde_redis_stream_interface::RedisStreamDeriveError::MissingFieldFromHashMap(String::from(#f_lit)))?;
                            let #f_ident = serde_json::from_str(&#f_ident).map_err(|_| serde_redis_stream_interface::RedisStreamDeriveError::DeserializationErrorFromJSON(String::from(#f_lit)))?;
                        )
                    }
                    ("json",true) => {
                        quote!(
                            let #f_ident: &redis::Value = map.get(#f_lit).ok();
                            let #f_ident = match #f_ident {
                                Some(#f_ident) => {
                                    let #f_ident = <String as redis::FromRedisValue>::from_redis_value(#f_ident).map_err(|_| serde_redis_stream_interface::RedisStreamDeriveError::MissingFieldFromHashMap(String::from(#f_lit)))?;
                                    serde_json::from_str(&#f_ident).map_err(|_| serde_redis_stream_interface::RedisStreamDeriveError::DeserializationErrorFromJSON(String::from(#f_lit)))?
                                }
                                None => {
                                    None
                                }
                            };   
                        )
                    }
                    _ => panic!("Invalid serialization method {}", serialization_method),
                }
            } else {
                match is_option{
                    true => {
                        quote!(
                            let #f_ident: &redis::Value = map.get(#f_lit).ok();
                            let #f_ident = match #f_ident {
                                Some(#f_ident) => {
                                    <#f_type as redis::FromRedisValue>::from_redis_value(#f_ident).map_err(|_| serde_redis_stream_interface::RedisStreamDeriveError::MissingFieldFromHashMap(String::from(#f_lit)))?
                                }
                                None => {
                                    None
                                }
                            };   
                        )                    
                    }
                    false => {
                        quote!(                        
                            let #f_ident: &redis::Value = map.get(#f_lit).ok_or_else(|| serde_redis_stream_interface::RedisStreamDeriveError::MissingFieldFromHashMap(String::from(#f_lit)))?;
                            let #f_ident = <#f_type as redis::FromRedisValue>::from_redis_value(#f_ident).map_err(|_| serde_redis_stream_interface::RedisStreamDeriveError::MissingFieldFromHashMap(String::from(#f_lit)))?;
                        )
                    }
                }
                
            }
        })
    .collect::<Vec<_>>();    

    let fields_struct = match data{
        syn::Data::Struct(s) => s.fields,
        syn::Data::Enum(_) => panic!("Not supported"),
        syn::Data::Union(_) => panic!("Not supported"),
    }
    .into_iter();
    
    let fields_struct = fields_struct
        .map(|field| {
            let f_ident = field.ident.expect("struct fields must have ident");
    
            quote!(                        
                #f_ident,
            )
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
            fn redis_deserialize(value: redis::streams::StreamKey) -> Result<Self, serde_redis_stream_interface::RedisStreamDeriveError> {
                let ids = value.ids;
                let map = &ids.first().ok_or(serde_redis_stream_interface::RedisStreamDeriveError::InvalidItemOnStreamKey)?.map;

                #(#fields_for_serialization)*
                
                #ident {
                    #(#fields_struct)*    
                }            
            }
        }
    };

    expanded.into()
}
