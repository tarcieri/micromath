/// Implements standard commutative operations such as Add and Mul on primitive types.
macro_rules! impl_commutative {
    ($vector:ident, $component:ident) => {
        impl Mul<$vector<$component>> for $component {
            type Output = $vector<$component>;

            fn mul(self, rhs: $vector<$component>) -> Self::Output {
                rhs.mul(self)
            }
        }
    };
}

pub(crate) use impl_commutative;
