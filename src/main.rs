use nvim_oxi::{self as oxi, api};

#[oxi::test]
fn set_get_del_var() {
    api::set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), api::get_var("foo"));
    assert_eq!(Ok(()), api::del_var("foo"));
}

fn main() {
    println!("Hello, world!");
}
