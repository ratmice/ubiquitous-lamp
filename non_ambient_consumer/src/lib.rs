use ambient_provider::foo as amb;
pub  fn whatever(foo: amb::Foo) {
        let _x = amb::Foo::new();
        let _y = amb::ambient_authority();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _x = amb::Foo::new();
        let x = amb::ambient_authority();
    }
}
