#![allow(incomplete_features)]
#![feature(adt_const_params)]

use core::marker::ConstParamTy;

#[cfg(feature = "self_rust_tokenize")]
use self_rust_tokenize::{SelfRustTokenize, quote, QuoteToTokens};

#[derive(Copy, Clone, PartialEq, Eq, ConstParamTy)]
pub enum COption<T: Copy>{
    Some(T),
    None
}

#[cfg(feature = "self_rust_tokenize")]
impl<T: Copy + QuoteToTokens> SelfRustTokenize for COption<T> {
    fn append_to_token_stream(&self, token_stream: &mut self_rust_tokenize::proc_macro2::TokenStream) {
        let iter = match self{
            COption::Some(s) => {
                let s = quote!(#s);
                quote!(COption::Some(#s))
            },
            COption::None => quote!(COption::None),
        };
        token_stream.extend(iter)
    }
}

impl<T: Copy> COption<T>{

    pub const fn into_std(&self) -> Option<T>{
        match self{
            COption::Some(t) => Option::Some(*t),
            COption::None => Option::None,
        }
    }
}