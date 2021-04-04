///! cuda-standard printf format string parser
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Expr, LitStr, Token,
};

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

#[derive(Debug, Clone, Copy)]
enum SizedType {
    I16,
    I32,
    I64,
    U16,
    U32,
    U64,
    Double,
    VoidPtr,
    StrPtr,
}

impl ToTokens for SizedType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            SizedType::I16 => quote!(i16),
            SizedType::I32 => quote!(i32),
            SizedType::I64 => quote!(i64),
            SizedType::U16 => quote!(u16),
            SizedType::U32 => quote!(u32),
            SizedType::U64 => quote!(u64),
            SizedType::Double => quote!(f64),
            SizedType::VoidPtr => quote!(*const ::core::ffi::c_void),
            SizedType::StrPtr => quote!(*const u8),
        })
    }
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
        _ => {
            return None;
        }
    })
}

struct FormatParser<'s> {
    iter: std::iter::Peekable<std::str::CharIndices<'s>>,
}

impl<'s> FormatParser<'s> {
    fn consume_if(&mut self, f: impl FnOnce(char) -> bool) -> bool {
        self.iter.next_if(|&(_, c)| f(c)).is_some()
    }

    fn consume_if_eq(&mut self, chr: char) -> bool {
        self.iter.next_if(|&(_, c)| c == chr).is_some()
    }

    fn consume_while(&mut self, mut f: impl FnMut(char) -> bool) {
        while let Some(_) = self.iter.next_if(|&(_, c)| f(c)) {}
    }

    fn consume_while_eq(&mut self, chr: char) {
        while let Some(_) = self.iter.next_if(|&(_, c)| c == chr) {}
    }
}

impl<'s> Iterator for FormatParser<'s> {
    type Item = std::result::Result<SizedType, (&'static str, usize, Option<usize>)>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (start, c) = self.iter.next()?;
            if c == '%' {
                // literal %
                if self.consume_if_eq('%') {
                    continue;
                }
                // flags
                self.consume_while(is_flag);
                // width
                if let Some((end, _)) = self.iter.next_if(|&(_, c)| c == '*') {
                    return Some(Err((
                        "variable width arguments are not yet supported",
                        start,
                        Some(end),
                    )));
                } else {
                    self.consume_while(|c| c.is_ascii_digit())
                }
                // prec
                if self.consume_if_eq('.') {
                    if let Some((end, _)) = self.iter.next_if(|&(_, c)| c == '*') {
                        return Some(Err((
                            "cuda does not support variable precision arguments",
                            start,
                            Some(end),
                        )));
                    } else {
                        self.consume_while(|c| c.is_ascii_digit())
                    }
                }
                // size
                // h|l|ll
                let size = if self.consume_if_eq('h') {
                    Some(FSize::Short)
                } else if self.consume_if_eq('h') {
                    Some(if self.consume_if_eq('h') {
                        FSize::LongLong
                    } else {
                        FSize::Long
                    })
                } else {
                    None
                };
                // type
                // cCdiouxXeEfgGaAnpsSZ
                let (end, c) = match self.iter.next() {
                    Some(v) => v,
                    None => {
                        return Some(Err(("ended early", start, None)));
                    }
                };
                let tyc = match type_char(c) {
                    Some(tyc) => tyc,
                    None => {
                        return Some(Err(("Invalid type specifier", start, Some(end))));
                    }
                };
                return Some(match (tyc, size) {
                    (FType::Integer { signed: true }, None) => Ok(SizedType::I32),
                    (FType::Integer { signed: true }, Some(FSize::Short)) => {
                        Err(("short int is unimplemented", start, Some(end)))
                    }
                    (FType::Integer { signed: true }, Some(FSize::Long)) => {
                        Err(("long int is not supported", start, Some(end)))
                    }
                    (FType::Integer { signed: true }, Some(FSize::LongLong)) => Ok(SizedType::I64),
                    (FType::Integer { signed: false }, None) => Ok(SizedType::U32),
                    (FType::Integer { signed: false }, Some(FSize::Short)) => Ok(SizedType::U16),
                    (FType::Integer { signed: false }, Some(FSize::Long)) => {
                        Err(("long unsigned int is unimplemented", start, Some(end)))
                    }
                    (FType::Integer { signed: false }, Some(FSize::LongLong)) => Ok(SizedType::U64),
                    (FType::Float, None) => Ok(SizedType::Double),
                    (FType::Float, Some(_)) => Err((
                        "Size specifiers are not allowed for floating-point arguments",
                        start,
                        Some(end),
                    )),
                    (FType::Char, None) => Ok(SizedType::U32),
                    (FType::Char, Some(_)) => Err((
                        "Size specifiers are not allowed for char arguments",
                        start,
                        Some(end),
                    )),
                    (FType::Str, None) => Ok(SizedType::StrPtr),
                    (FType::Str, Some(_)) => Err((
                        "Size specifiers are not allowed for str arguments",
                        start,
                        Some(end),
                    )),
                    (FType::Pointer, None) => Ok(SizedType::VoidPtr),
                    (FType::Pointer, Some(_)) => Err((
                        "Size specifiers are not allowed for pointer arguments",
                        start,
                        Some(end),
                    )),
                });
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
        Ok(Self { format, args })
    }
}

impl ToTokens for VPrintfCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let VPrintfCall { format, args } = self;
        if args.len() > 32 {
            eprintln!("Warning: cuda printf can accept at most 32 arguments in addition to the format string\nsee (https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#limitations)")
        }
        let fmt = format.value();
        let ires = FormatParser {
            iter: fmt.char_indices().peekable(),
        }
        .collect::<std::result::Result<Vec<SizedType>, _>>();
        let type_specifiers = match ires {
            Ok(v) => v,
            Err((msg, start, end)) => {
                let substr = end.map_or_else(|| &fmt[start..], |end| &fmt[start..=end]);
                let error = format!("{}: \"{}\"", msg, substr);
                tokens.extend(syn::Error::new_spanned(format, error).into_compile_error());
                return;
            }
        };
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
