extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _ = input;

    let input = parse_macro_input!(input as DeriveInput);

    let structname = input.ident;
    let structbuildername = format_ident!("{}Builder", structname);

    match input.data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(f) => {
                for i in f.named.iter() {
                    eprintln!("{:#?}", i.ident.as_ref().unwrap());
                    eprintln!("{:#?}", i.ty.to_owned().to_token_stream());
                }
            }
            syn::Fields::Unnamed(_) => todo!(),
            syn::Fields::Unit => todo!(),
        },
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    }

    let output_tokens = quote! {
        pub struct #structbuildername {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #structname {
            pub fn builder() -> #structbuildername {
                #structbuildername {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }

        impl #structbuildername {
            fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }
         }

    };

    TokenStream::from(output_tokens)
}
