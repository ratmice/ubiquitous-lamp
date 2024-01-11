pub fn something() -> ambient_provider::foo::Foo {
  ambient_provider::foo::Foo::new_ambient(ambient_provider::foo::ambient_authority())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let foo = ambient_provider::bar::Bar::new_ambient(ambient_provider::bar::ambient_authority());
    }
}
