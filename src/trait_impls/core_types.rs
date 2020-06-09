use crate::{FloatDiff, FloatEq, FloatEqAll, FloatEqAllDebug, FloatEqDebug, FloatUlps, Ulps};

//------------------------------------------------------------------------------
// ref types
//------------------------------------------------------------------------------
macro_rules! impl_for_refs {
    // $a and $b are mutability
    (&$($a:ident)?, &$($b:ident)?) => {
        impl<A: ?Sized, B: ?Sized> FloatDiff<&$($b)? B> for &$($a)? A
        where
            A: FloatDiff<B>,
        {
            type Output = A::Output;

            #[inline]
            fn abs_diff(&self, other: &&$($b)? B) -> Option<Self::Output> {
                FloatDiff::abs_diff(*self, *other)
            }

            #[inline]
            fn ulps_diff(&self, other: &&$($b)? B) -> Option<Ulps<Self::Output>> {
                FloatDiff::ulps_diff(*self, *other)
            }
        }

        impl<A: ?Sized, B: ?Sized> FloatEq<&$($b)? B> for &$($a)? A
        where
            A: FloatEq<B>,
        {
            type Epsilon = A::Epsilon;

            #[inline]
            fn eq_abs(&self, other: &&$($b)? B, max_diff: &Self::Epsilon) -> bool {
                FloatEq::eq_abs(*self, *other, max_diff)
            }

            #[inline]
            fn eq_rel(&self, other: &&$($b)? B, max_diff: &Self::Epsilon) -> bool {
                FloatEq::eq_rel(*self, *other, max_diff)
            }

            #[inline]
            fn eq_ulps(&self, other: &&$($b)? B, max_diff: &Ulps<Self::Epsilon>) -> bool {
                FloatEq::eq_ulps(*self, *other, max_diff)
            }
        }

        impl<A: ?Sized, B: ?Sized> FloatEqAll<&$($b)? B> for &$($a)? A
        where
            A: FloatEqAll<B>,
        {
            type Epsilon = A::Epsilon;

            #[inline]
            fn eq_abs_all(&self, other: &&$($b)? B, max_diff: &Self::Epsilon) -> bool {
                FloatEqAll::eq_abs_all(*self, *other, max_diff)
            }

            #[inline]
            fn eq_rel_all(&self, other: &&$($b)? B, max_diff: &Self::Epsilon) -> bool {
                FloatEqAll::eq_rel_all(*self, *other, max_diff)
            }

            #[inline]
            fn eq_ulps_all(&self, other: &&$($b)? B, max_diff: &Ulps<Self::Epsilon>) -> bool {
                FloatEqAll::eq_ulps_all(*self, *other, max_diff)
            }
        }

        impl<A: ?Sized, B: ?Sized> FloatEqDebug<&$($b)? B> for &$($a)? A
        where
            A: FloatEqDebug<B>,
        {
            type DebugEpsilon = A::DebugEpsilon;

            #[inline]
            fn debug_abs_epsilon(&self, other: &&$($b)? B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
                FloatEqDebug::debug_abs_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_rel_epsilon(&self, other: &&$($b)? B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
                FloatEqDebug::debug_rel_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_ulps_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Ulps<Self::Epsilon>,
            ) -> Ulps<Self::DebugEpsilon> {
                FloatEqDebug::debug_ulps_epsilon(*self, *other, max_diff)
            }
        }

        impl<A: ?Sized, B: ?Sized> FloatEqAllDebug<&$($b)? B> for &$($a)? A
        where
            A: FloatEqAllDebug<B>,
        {
            type DebugEpsilon = A::DebugEpsilon;

            #[inline]
            fn debug_abs_all_epsilon(&self, other: &&$($b)? B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
                FloatEqAllDebug::debug_abs_all_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_rel_all_epsilon(&self, other: &&$($b)? B, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
                FloatEqAllDebug::debug_rel_all_epsilon(*self, *other, max_diff)
            }

            #[inline]
            fn debug_ulps_all_epsilon(
                &self,
                other: &&$($b)? B,
                max_diff: &Ulps<Self::Epsilon>,
            ) -> Ulps<Self::DebugEpsilon> {
                FloatEqAllDebug::debug_ulps_all_epsilon(*self, *other, max_diff)
            }
        }
    };
}

impl_for_refs!(&, &);
impl_for_refs!(&, &mut);
impl_for_refs!(&mut, &);
impl_for_refs!(&mut, &mut);

//------------------------------------------------------------------------------
// Option
//
// Note: The Option impls are over `impl<T>` and not `impl<A, B>` since that breaks
// type inference and makes it harder to use `None`.
//------------------------------------------------------------------------------
impl<T: FloatUlps> FloatUlps for Option<T> {
    type Ulps = Option<Ulps<T>>;
}

impl<T: FloatDiff> FloatDiff for Option<T> {
    type Output = T::Output;

    #[inline]
    fn abs_diff(&self, other: &Option<T>) -> Option<Self::Output> {
        FloatDiff::abs_diff(self.as_ref()?, other.as_ref()?)
    }

    #[inline]
    fn ulps_diff(&self, other: &Option<T>) -> Option<Ulps<Self::Output>> {
        FloatDiff::ulps_diff(self.as_ref()?, other.as_ref()?)
    }
}

impl<T: FloatEq> FloatEq for Option<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn eq_abs(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> bool {
        self.is_some()
            && other.is_some()
            && FloatEq::eq_abs(self.as_ref().unwrap(), other.as_ref().unwrap(), max_diff)
    }

    #[inline]
    fn eq_rel(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> bool {
        self.is_some()
            && other.is_some()
            && FloatEq::eq_rel(self.as_ref().unwrap(), other.as_ref().unwrap(), max_diff)
    }

    #[inline]
    fn eq_ulps(&self, other: &Option<T>, max_diff: &Ulps<Self::Epsilon>) -> bool {
        self.is_some()
            && other.is_some()
            && FloatEq::eq_ulps(self.as_ref().unwrap(), other.as_ref().unwrap(), max_diff)
    }
}

impl<T: FloatEqAll> FloatEqAll<Option<T>> for Option<T> {
    type Epsilon = T::Epsilon;

    #[inline]
    fn eq_abs_all(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> bool {
        self.is_some()
            && other.is_some()
            && FloatEqAll::eq_abs_all(self.as_ref().unwrap(), other.as_ref().unwrap(), max_diff)
    }

    #[inline]
    fn eq_rel_all(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> bool {
        self.is_some()
            && other.is_some()
            && FloatEqAll::eq_rel_all(self.as_ref().unwrap(), other.as_ref().unwrap(), max_diff)
    }

    #[inline]
    fn eq_ulps_all(&self, other: &Option<T>, max_diff: &Ulps<Self::Epsilon>) -> bool {
        self.is_some()
            && other.is_some()
            && FloatEqAll::eq_ulps_all(self.as_ref().unwrap(), other.as_ref().unwrap(), max_diff)
    }
}

impl<T: FloatEqDebug> FloatEqDebug for Option<T> {
    type DebugEpsilon = Option<T::DebugEpsilon>;

    #[inline]
    fn debug_abs_epsilon(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        Some(FloatEqDebug::debug_abs_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff,
        ))
    }

    #[inline]
    fn debug_rel_epsilon(&self, other: &Option<T>, max_diff: &Self::Epsilon) -> Self::DebugEpsilon {
        Some(FloatEqDebug::debug_rel_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff,
        ))
    }

    #[inline]
    fn debug_ulps_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        Some(FloatEqDebug::debug_ulps_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff,
        ))
    }
}

impl<T: FloatEqAllDebug> FloatEqAllDebug for Option<T> {
    type DebugEpsilon = Option<T::DebugEpsilon>;

    #[inline]
    fn debug_abs_all_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        Some(FloatEqAllDebug::debug_abs_all_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff,
        ))
    }

    #[inline]
    fn debug_rel_all_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Self::Epsilon,
    ) -> Self::DebugEpsilon {
        Some(FloatEqAllDebug::debug_rel_all_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff,
        ))
    }

    #[inline]
    fn debug_ulps_all_epsilon(
        &self,
        other: &Option<T>,
        max_diff: &Ulps<Self::Epsilon>,
    ) -> Ulps<Self::DebugEpsilon> {
        Some(FloatEqAllDebug::debug_ulps_all_epsilon(
            &self.as_ref()?,
            &other.as_ref()?,
            max_diff,
        ))
    }
}
