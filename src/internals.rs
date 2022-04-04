use core::{marker::PhantomData, mem::ManuallyDrop};

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct Usize<const N: usize>;

impl<const N: usize> Usize<N> {
    #[inline(always)]
    pub const fn infer_mda<T>(self, _: &ManuallyDrop<[T; N]>) {}

    #[inline(always)]
    pub const fn get(self) -> usize {
        N
    }
}

///
/// # Safety
///
/// Implementors must have exactly one type parameter,
/// and the `T` associated type must be the value of that type parameter.
#[doc(hidden)]
pub unsafe trait GetTypeParam: Sized {
    type T;

    const PROOF: TypeParam<Self, Self::T> = TypeParam { types: PhantomData };
}

pub struct TypeParam<S, T> {
    types: PhantomData<fn() -> (PhantomData<S>, PhantomData<T>)>,
}

impl<S, T> Copy for TypeParam<S, T> {}

impl<S, T> Clone for TypeParam<S, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, T> TypeParam<S, T> {
    #[inline(always)]
    pub const fn assert_type_param(self, _: PhantomData<T>) {}

    pub const unsafe fn new_unchecked() -> Self {
        Self::__NEW
    }

    const __NEW: Self = Self { types: PhantomData };
}

pub unsafe trait ArrayLength {
    const LENGTH: usize;
}

unsafe impl<T, const L: usize> GetTypeParam for [T; L] {
    type T = T;
}

unsafe impl<T, const L: usize> ArrayLength for [T; L] {
    const LENGTH: usize = L;
}

#[repr(transparent)]
pub struct Identity<T> {
    pub inner: T,
}

#[repr(transparent)]
pub struct ArrayAndGhost<T, const LEN: usize> {
    pub inner: [T; LEN],
    pub elem_ty: PhantomData<T>,
}

#[doc(hidden)]
pub const unsafe fn concat_arrays<From_, T, const CONCAT_LEN: usize>(
    this: From_,
    _param: TypeParam<From_, T>,
) -> [T; CONCAT_LEN] {
    use core::mem::size_of;

    assert!(size_of::<From_>() == size_of::<[T; CONCAT_LEN]>());

    const_transmute!(
        ArrayHList<Rem, T, LEN>,
        [T; CONCAT_LEN],
        this
    )
}

/// Helper type for transmuting non-Copy types without adding any overhead in debug builds.
///
#[doc(hidden)]
#[repr(C)]
pub union TransmuterMD<T, U> {
    pub from: ManuallyDrop<T>,
    pub to: ManuallyDrop<U>,
}

macro_rules! const_transmute {
    ($from:ty, $to:ty, $val:expr) => {
        $crate::__::ManuallyDrop::into_inner(
            $crate::__::TransmuterMD {
                from: $crate::__::ManuallyDrop::new($val),
            }
            .to,
        )
    };
}
use const_transmute;
