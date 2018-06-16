#[macro_use]
extern crate proc_macro_hack;

proc_macro_expr_impl! {
    pub fn unhygienic_impl(input: &str) -> String {
        input.to_string()
    }
}

proc_macro_item_impl! {
    pub fn unhygienic_item_impl(input: &str) -> String {
        input.to_string()
    }
}
