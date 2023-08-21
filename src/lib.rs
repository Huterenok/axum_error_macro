use litrs::{IntegerLit, StringLit};
use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Fields, MetaList, Variant};

#[proc_macro_derive(IntoResponse, attributes(error))]
pub fn axum_error_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_error(ast)
}

fn impl_error(ast: syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let eident = Ident::new(&format!("Derive{}", ident.to_string()), ident.span());

    let variants = retrieve_variants(&ast);
    let matches = match_error(ident.clone(), eident.clone(), variants);

    let expanded = quote!(
          use axum::response::IntoResponse;
                    use axum::http::StatusCode;
                use hyper::body::HttpBody;

          pub struct #eident(axum::http::StatusCode, String);

          impl #eident {
            pub fn new(code: u16, msg: String) -> Self {
               let code = axum::http::StatusCode::from_u16(code).expect("Error code must be valid");
               #eident(code, msg)
            }
          }

          impl axum::response::IntoResponse for #eident {
            fn into_response(self) -> axum::response::Response {
                axum::response::Response::builder()
                  .status(self.0)
                  .header(
                    hyper::header::CONTENT_TYPE,
                    axum::http::HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                  )
                  .body(axum::body::boxed(axum::body::Full::from(self.1)))
                  .unwrap()
            }
          }

            impl axum::response::IntoResponse for #ident {
                fn into_response(self) -> axum::response::Response {
                    let res = match self {
                      #(#matches),*
                    };
                    res.into_response()
                }
            }
    );

    expanded.into()
}

fn retrieve_variants(ast: &syn::DeriveInput) -> Punctuated<Variant, Comma> {
    if let syn::Data::Enum(syn::DataEnum { variants, .. }) = &ast.data {
        return variants.to_owned();
    } else {
        panic!("Expected at least one enum variant")
    };
}

fn retrieve_params(fields: Fields) -> Option<Vec<proc_macro2::TokenStream>> {
    if let syn::Fields::Unnamed(pfields) = fields {
        if pfields.unnamed.len() == 0 {
            None
        } else {
            Some(
                pfields
                    .unnamed
                    .into_iter()
                    .enumerate()
                    .map(|(index, field)| {
                        let unique_param_ident = Ident::new(
                            &format!("param_{}", char::from_u32(index as u32 + 97).unwrap()),
                            field.span(),
                        );
                        quote!(#unique_param_ident)
                    })
                    .collect(),
            )
        }
    } else {
        None
    }
}

fn match_error(
    ident: proc_macro2::Ident,
    eident: proc_macro2::Ident,
    variants: Punctuated<Variant, Comma>,
) -> Vec<proc_macro2::TokenStream> {
    variants
        .iter()
        .filter_map(|v| {
            let bident = &v.ident;
            if let syn::Meta::List(meta_list) = &v.attrs[0].meta {
                let (error_code, error_msg) = validate_attribute(meta_list);

                let retrieved_params = retrieve_params(v.fields.clone());
                let (match_params, format_params) = if let Some(params) = retrieved_params {
                    (Some(quote!((#(#params),*))), Some(quote!(#(#params),*)))
                } else {
                    (None, None)
                };

                return Some(quote!(
                  #ident::#bident #match_params => {
                    #eident::new(#error_code, format!(#error_msg, #format_params))
                  }
                ));
            } else {
                None
            }
        })
        .collect()
}

fn validate_attribute(meta_list: &MetaList) -> (u16, String) {
    let mut tokens = meta_list.tokens.to_owned().into_iter();
    match tokens.next().unwrap() {
        TokenTree::Ident(ref i) => assert_eq!(i, "code"),
        tt => panic!("Expected 'code', found {}", tt),
    };
    match tokens.next().unwrap() {
        TokenTree::Punct(ref i) => assert_eq!(i.as_char(), '='),
        tt => panic!("Expected '=' , found {}", tt),
    };
    let error_code: u16 = match tokens.next().unwrap() {
        TokenTree::Literal(ref i) => {
            let int_lit = match IntegerLit::try_from(i) {
                Err(e) => panic!("{}", e.to_compile_error()),
                Ok(lit) => lit,
            };
            int_lit.value().unwrap()
        }
        tt => panic!("Expected http code error, found {}", tt),
    };
    match tokens.next().unwrap() {
        TokenTree::Punct(ref i) => assert_eq!(i.as_char(), ','),
        tt => panic!("Expected ',' , found {}", tt),
    };
    match tokens.next().unwrap() {
        TokenTree::Ident(ref i) => assert_eq!(i, "msg"),
        tt => panic!("Expected 'msg', found {}", tt),
    };
    match tokens.next().unwrap() {
        TokenTree::Punct(ref i) => assert_eq!(i.as_char(), '='),
        tt => panic!("Expected '=' , found {}", tt),
    };
    let error_msg = match tokens.next().unwrap() {
        TokenTree::Literal(ref i) => {
            let string_lit = match StringLit::try_from(i) {
                Err(e) => panic!("{}", e.to_compile_error()),
                Ok(lit) => lit,
            };
            string_lit.value().to_string()
        }
        tt => panic!("Expected error message, found {}", tt),
    };

    (error_code, error_msg)
}