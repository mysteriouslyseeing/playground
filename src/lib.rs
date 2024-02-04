trait Foo {
    fn bar();
}

macro_rules! problem {
    ($ty:ident) => {
        impl<$ty: Foo> Foo for ($ty,) {
            fn bar() { <$ty>::bar() }
        }
    };
    ($ty:ident $(, $rest:ident)*) => {
        impl<$ty: Foo, $($rest: Foo),*> Foo for ($ty, $($rest),*) {
            fn bar() {
                <($($rest),*)>::bar()
            }
        }
    }
}

problem!(T1, T2);
