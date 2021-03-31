use proc_macro::{TokenStream};
use quote::{ToTokens};
use syn::{parse_macro_input};

mod parser;
use crate::parser::VPrintfCall;

/// cuda-standard printf
/// specification https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#formatted-output
///
/// format = %[flags][width][.precision][size]type
/// flags = '#' | ' ' | '0' | '+' | '-'
/// width: '*' | '0-9'
/// precision: '0-9'
/// size: 'h' | 'l' | 'll'
/// type: "%cdiouxXpeEfgGaAs"
///
/// Allowed Size + Type specifiers
/// "%[uoxX]" => u32
/// "%h[uoxX]" => u16
/// "%ll[uoxX]" => u64
/// "%[di]" => i32
/// "%ll[di]" => i64
/// "%[eEfgGaA]" => f64
/// "%c" => u32
/// "%s" => *const u8
/// "%p" => *const ::core::ffi::c_void
///
/// Warning: '*' as the width argument is not yet implemented
/// Warning: "%%" will be the only accepted way to print a literal '%', i.e. "%-03l%" will not be allowed!
/// Warning: Due to the host-platform dependence on the size of long-integers, %ld & %lu will not be allowed!
/// Warning: Due to the host-platform dependence on wide-chars, %lc & %ls will not be allowed!
#[proc_macro]
pub fn printf(input: TokenStream) -> TokenStream {
    TokenStream::from(parse_macro_input!(input as VPrintfCall).into_token_stream())
}
