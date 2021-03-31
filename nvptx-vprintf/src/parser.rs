///! cuda-standard printf format string parser

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::{Parse, ParseStream, Result}, LitStr, Expr, punctuated::Punctuated, Token};

fn is_flag(c: char) -> bool {
    matches!(c, '-' | '+' | '0' | ' ' | '#')
}

#[allow(non_camel_case_types)]
enum FSize {
    Short,
    Long,
    LongLong,
}

enum FType {
    Integer { signed: bool },
    Float,
    Char,
    Pointer,
    Str,
}

fn type_char(c: char) -> Option<FType> {
    // cdiouxXpeEfgGaAs
    Some(match c {
        'd' | 'i' => FType::Integer { signed: true },
        'u' | 'o' | 'x' | 'X' => FType::Integer { signed: false },
        'f' | 'F' | 'e' | 'E' | 'g' | 'G' | 'a' | 'A' => FType::Float,
        'c' => FType::Char,
        's' => FType::Str,
        'p' => FType::Pointer,
        _ => {return None;},
    })
}

struct FormatParser<'s> {
    iter: std::iter::Peekable<std::str::Chars<'s>>,
}

impl<'s> Iterator for FormatParser<'s> {
    type Item = syn::Type;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.iter.next()? == '%' {
                // literal %
                if let Some(_) = self.iter.next_if_eq(&'%') {
                    continue;
                }
                // flags
                while let Some(_) = self.iter.next_if(|&c| is_flag(c)) {}
                // width
                if let None = self.iter.next_if_eq(&'*') {
                    while let Some(_) = self.iter.next_if(|c| c.is_ascii_digit()) {}
                } else {
                    unimplemented!("varible width arguments are not yet supported")
                }
                // prec
                if let Some(_) = self.iter.next_if_eq(&'.') {
                    if let Some(_) = self.iter.next_if_eq(&'*') {
                        panic!("cuda does not support varible precision arguments")
                    }
                    while let Some(_) = self.iter.next_if(|c| c.is_ascii_digit()) {}
                }
                // size
                // h|l|ll
                let size = if let Some(_) = self.iter.next_if_eq(&'h') {
                    Some(FSize::Short)
                } else if let Some(_) = self.iter.next_if_eq(&'h') {
                    Some(if let Some(_) = self.iter.next_if_eq(&'h') {
                        FSize::LongLong
                    } else {
                        FSize::Long
                    })
                } else {
                    None
                };
                // type
                // cCdiouxXeEfgGaAnpsSZ
                return Some(match (type_char(self.iter.next()?).expect("No Type Specifier given"), size) {
                    (FType::Integer { signed: true }, None) => syn::parse_quote!(i32),
                    (FType::Integer { signed: true }, Some(FSize::Short)) => unimplemented!(""),
                    (FType::Integer { signed: true }, Some(FSize::Long)) => panic!(""),
                    (FType::Integer { signed: true }, Some(FSize::LongLong)) => syn::parse_quote!(i64),
                    (FType::Integer { signed: false }, None) => syn::parse_quote!(u32),
                    (FType::Integer { signed: false }, Some(FSize::Short)) => syn::parse_quote!(u16),
                    (FType::Integer { signed: false }, Some(FSize::Long)) => unimplemented!(""),
                    (FType::Integer { signed: false }, Some(FSize::LongLong)) => syn::parse_quote!(u64),
                    (FType::Float, None) => syn::parse_quote!(f64),
                    (FType::Float, Some(_)) => panic!("Size specfiers are not allowed for floating-point arguments"),
                    (FType::Char, None) => syn::parse_quote!(u32),
                    (FType::Char, Some(_)) => panic!("Size specfiers are not allowed for char arguments"),
                    (FType::Str, None) => syn::parse_quote!(*const u8),
                    (FType::Str, Some(_)) => panic!("Size specfiers are not allowed for str arguments"),
                    (FType::Pointer, None) => syn::parse_quote!(*const core::ffi::c_void),
                    (FType::Pointer, Some(_)) => panic!("Size specfiers are not allowed for pointer arguments"),
                })
            }
        }
    }
}


pub struct VPrintfCall {
    format: LitStr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for VPrintfCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let format = input.parse()?;
        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }
        let args = input.parse_terminated(Expr::parse)?;
        Ok(Self {format, args, })
    }
}

impl ToTokens for VPrintfCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let VPrintfCall { format, args } = self;
        if args.len() > 32 {
            eprintln!("Warning: cuda printf can accept at most 32 arguments in addition to the format string\nsee (https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#limitations)")
        }
        let fmt = format.value();
        let type_specifiers = FormatParser {
            iter: fmt.chars().peekable()
        }.collect::<Vec<_>>();
        assert_eq!(type_specifiers.len(), args.len());
        let args = args.into_iter();
        tokens.extend(quote! {
            {
                #[repr(C)]
                struct VPrintfArgs( #( #type_specifiers ),* );
                let mut vargs = VPrintfArgs( #( #args ),* );
                ::nvptx_sys::vprintf( (#format).as_ptr(), ::core::ptr::addr_of_mut!(vargs) as *mut ::core::ffi::c_void)
            }
        })
    }
}
