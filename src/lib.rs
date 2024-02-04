pub trait Token {
    fn repr(&self) -> impl std::ops::Deref<Target = str>;
}
pub trait ToExpected<T: Token> {
    fn expect() -> impl Iterator<Item = T>;
}

macro_rules! impl_to_expected_tuple {
    ($ty:ident $(,)?) => {
        impl<T: Token, $ty: ToExpected<T>> ToExpected<T> for ($ty,) {
            fn expect() -> impl Iterator<Item = T> {
                <$ty>::expect()
            }
        }
    };
    ($ty:ident $(, $rest:ident)* $(,)?) => {
        impl<T: Token, $ty: ToExpected<T>, $($rest: ToExpected<T>),*> ToExpected<T> for ($ty, $($rest),*) {
            fn expect() -> impl Iterator<Item = T> {
                <$ty>::expect().chain(<($($rest),*)>::expect())
            }
        }

        impl_to_expected_tuple!($($rest),*);
    }
}

impl_to_expected_tuple!(T1, T2, T3, T4);
