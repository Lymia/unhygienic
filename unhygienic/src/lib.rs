#![no_std]

//! A dead simple macro to erase macro hygiene. This stringifies the input, then reparses it in
//! the caller's context (with a few warts.)

#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate unhygienic_impl;
#[doc(hidden)]
pub use unhygienic_impl::*;

proc_macro_expr_decl! {
    /// Erases the hygiene from its parameters, causing it to use the context of the call site.
    /// The input must be an expression.
    unhygienic! => unhygienic_impl
}

proc_macro_item_decl! {
    /// Erases the hygiene from its parameters, causing it to use the context of the call site.
    /// The input must be zero, one, or more items.
    unhygienic_item! => unhygienic_item_impl
}