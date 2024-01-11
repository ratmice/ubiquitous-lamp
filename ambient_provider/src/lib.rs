/// There are 3 options really, depending upon the situation:
/// The first just spay #[cfg(feature = "ambient-authority")] where-ever you can,
/// it might not be an option, and is tedious and ugly, and seems overzealous.

/// 1. import the type, conditionally import `fn ambient_authority`.
pub mod foo {
    pub use ambient_authority::AmbientAuthority;

    // Re-export ambient_authority etc. so that users can use our version.
    #[doc(hidden)]
    #[cfg(feature = "ambient-authority-comptime")]
    pub use ambient_authority::ambient_authority_known_at_compile_time;

    #[cfg(feature = "ambient-authority")]
    pub use ambient_authority::ambient_authority;

    pub struct Foo(Option<AmbientAuthority>);
    impl Foo {
       pub fn new() -> Self {
           Self(None)
       }
       pub fn new_ambient(ambient_authority: AmbientAuthority) -> Self {
         Self(Some(ambient_authority))
       }
       pub fn hello(&self) {
           println!("hello");
       }
    }
}

/// 2. import type & fn into a private module, then conditionally export publicly
pub mod bar {
    #[allow(unused_imports)]
    mod _ambient {
        #[doc(hidden)]
        pub use ambient_authority::ambient_authority_known_at_compile_time;
        // In theory this solution shouldn't be necessary, because callers
        // should be passing in ambient authority, rather than the library ever calling it.
        //
        // This would be the case if you had something like 
        // `unsafe fn foo() { crate::_ambient::ambient_authority() }`, you can refer to it privately
        // and still conditionally export it publicly, but it seems like that defeats the purpose. 
        #[allow(unused_imports)]
        pub use ambient_authority::ambient_authority;
    }

    pub use ambient_authority::AmbientAuthority;
    /// Re-export these publicly if the feature is enabled.
    #[cfg(feature = "ambient-authority")]
    pub use _ambient::ambient_authority;
    #[cfg(feature = "ambient-authority-comptime")]
    pub use _ambient::ambient_authority_known_at_compile_time;

    // Whether or not the feature is enabled, we can refer to the type and stuff.
    pub struct Bar(Option<AmbientAuthority>);

    impl Bar {
       pub fn new() -> Self {
           Self(None) 
       }

       pub fn new_ambient(ambient_authority: AmbientAuthority) -> Self { 
         Self(Some(ambient_authority))
       }
       pub fn hello(&self) {
           println!("hello");
       }
    }
}
