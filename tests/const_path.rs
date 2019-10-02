#![no_std]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate serde_big_array;

mod module {
    pub const NUMBER: usize = 127;
}

big_array! { BigArray; module::NUMBER, }

#[derive(Serialize, Deserialize)]
struct S {
    #[serde(with = "BigArray")]
    arr: [u8; module::NUMBER],
}

#[test]
fn test() {
    let s = S { arr: [1; module::NUMBER] };
    let j = serde_json::to_string(&s).unwrap();
    let s_back = serde_json::from_str::<S>(&j).unwrap();
    assert!(&s.arr[..] == &s_back.arr[..]);
}