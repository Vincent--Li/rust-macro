use macros::AutoDeref;

#[derive(Debug, AutoDeref)]
#[deref(field = "inner", mutable = true)]
pub struct RespBulkString {
    inner: String,
    nothing: (),
}

fn main() {
    let mut s = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
    };
    s.push_str(" world");
    println!("{:?}", s);
}
