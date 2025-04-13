extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(QueueDrop)]
pub fn derive_queue_drop(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = input.ident;
    match input.data {
        syn::Data::Struct(data_struct) => {
            let fields_code =  match data_struct.fields {
                syn::Fields::Named(fields_named) => {
                    let iter = fields_named.named.into_iter().map(|f| f.ident).rev();
                    quote! {
                        #( queue.push(&mut self.#iter as &mut dyn ::queue_drop::QueueDrop); )*
                    }
                }
                syn::Fields::Unnamed(fields_unnamed) => {
                    let iter = fields_unnamed
                        .unnamed
                        .into_iter()
                        .enumerate()
                        .map(|(i, _)| syn::Index::from(i)).rev();
                    quote! {
                        #( queue.push(&mut self.#iter as &mut dyn ::queue_drop::QueueDrop); )*
                    }
                }
                syn::Fields::Unit => quote! {},
            };
            quote! {
                impl ::queue_drop::QueueDrop for #ident {
                    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
                        ::queue_drop::SideDrop::drop(self);
                        #fields_code
                    }
                }
            }
        },
        syn::Data::Enum(data_enum) => {
            let iter = data_enum.variants.into_iter().map(|v| {
                let ident = v.ident;
                let fields_code = match v.fields {
                    syn::Fields::Named(fields_named) => {
                        let ident_stream = fields_named.named.into_iter().map(|f| f.ident);
                        let ident_stream_clone = ident_stream.clone().rev();
                        quote! {
                            { #(#ident_stream),* } => {
                                #(queue.push(#ident_stream_clone as &mut dyn ::queue_drop::QueueDrop);)*
                            }
                        }
                    },
                    syn::Fields::Unnamed(fields_unnamed) => {
                        let ident_stream = (0..fields_unnamed.unnamed.len()).into_iter().map(|n| syn::Ident::new(&format!("var{n}"), proc_macro2::Span::call_site()));
                        let ident_stream_clone = ident_stream.clone().rev();
                        quote! {
                            ( #(#ident_stream),* ) => {
                                #(queue.push(#ident_stream_clone as &mut dyn ::queue_drop::QueueDrop);)*
                            }
                        }
                    },
                    syn::Fields::Unit => quote! { => () },
                };
                quote! {
                    #ident #fields_code
                }
            });
            quote! {
                impl ::queue_drop::QueueDrop for #ident {
                    unsafe fn queue_drop<'a>(self: &'a mut Self, queue: &mut Vec<&'a mut dyn QueueDrop>) {
                        ::queue_drop::SideDrop::drop(self);
                        match self {
                            #(Self:: #iter),*
                        }
                    }
                }
            }
        },
        syn::Data::Union(_) => panic!("Cannot derive QueueDrop for unions"),
    }
    .into()
}

#[proc_macro_attribute]
pub fn side_drop(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let origin: proc_macro2::TokenStream = input.clone().into();
    let mut input = syn::parse_macro_input!(input as syn::ItemImpl);
    input.trait_ = Some((
        None,
        syn::parse2(quote::quote!(::queue_drop::SideDrop)).unwrap(),
        syn::token::For(proc_macro2::Span::call_site()),
    ));
    match input.items.iter_mut().next() {
        Some(syn::ImplItem::Fn(impl_item_fn)) => {
            impl_item_fn.sig.unsafety = Some(syn::token::Unsafe(proc_macro2::Span::call_site()))
        }
        _ => (),
    }
    quote! {
        #origin
        #input
    }
    .into()
}
