
#[cfg(test)]
mod tests;

mod internals;

#[doc(hidden)]
pub mod __ {
    pub use core::{
        marker::PhantomData,
        mem::ManuallyDrop,
    };

    pub use crate::internals::*;
}

#[macro_export]
macro_rules! concat_arrays {
    () => ([]);
    ( $($args:tt)* ) => (
        $crate::__concat_arrays_preprocess_inner!{() ($($args)*)}
    )
}


#[doc(hidden)]
#[macro_export]
macro_rules! __concat_arrays_preprocess_inner {
    (
        (
            $(
                (
                    $expr:expr,
                    ($($elem:ty)?),
                    ($len:expr),
                    ($($type:tt)*),
                )
            )*
        )
        ($(,)?)
    ) => {
        unsafe{
            #[repr(C, packed)]
            struct __Concater<__PrivT>(
                $([__PrivT; $len],)*
            );

            impl<__PrivT> __Concater<__PrivT> {
                const PROOF: $crate::__::TypeParam<Self, __PrivT> = unsafe{
                    $crate::__::TypeParam::new_unchecked() 
                };
            }

            $crate::__::concat_arrays::<
                _,
                _,
                {
                    let mut len = 0;
                    $( len += $len; )*
                    len
                }
            >(
                __Concater(
                    $($crate::__type_ascription!(($expr) ($($type)*)),)*
                ),
                __Concater::PROOF,
            )
        }
    };

    (
        ($($prev:tt)*)
        ( [$($array:tt)*] $(: [$elem_ty:ty; $($len:tt)*])?  $(, $($rem:tt)*)? )
    ) => {
        $crate::__concat_arrays_preprocess_inner!{
            (
                $($prev)*
                (
                    [$($array)*],
                    ($($elem_ty)?),
                    ($crate::__get_array_length!($($array)*)),
                    ($([$elem_ty; $($len)*])?),
                )
            )
            ($($($rem)*)?)
        }
    };

    (
        ($($prev:tt)*)
        ( [$($array:tt)*] $(: $type:ty)?  $(, $($rem:tt)*)? )
    ) => {
        $crate::__concat_arrays_preprocess_inner!{
            (
                $($prev)*
                (
                    [$($array)*],
                    ($(<$type as $crate::__::GetTypeParam>::T)?),
                    ($crate::__get_array_length!($($array)*)),
                    ($($type)?),
                )
            )
            ($($($rem)*)?)
        }
    };


    (
        ($($prev:tt)*)
        ( $expr:tt $(: [$elem_ty:ty; $($len:tt)*])?  $(, $($rem:tt)*)? )
    ) => {
        $crate::__concat_arrays_preprocess_inner!{
            (
                $($prev)*
                (
                    $expr,
                    ($($elem_ty)?),
                    ($crate::__length_or_infer!(($expr), ($($elem_ty)?),($(const $($len)*)?))),
                    ($([$elem_ty; $($len)*])?),
                )
            )
            ($($($rem)*)?)
        }
    };

    (
        ($($prev:tt)*)
        ( $expr:tt $(: $type:ty)?  $(, $($rem:tt)*)? )
    ) => {
        $crate::__concat_arrays_preprocess_inner!{
            (
                $($prev)*
                (
                    $expr,
                    ($(<$type as $crate::__::GetTypeParam>::T)?),
                    ($crate::__length_or_infer!(($expr), (), $((type $type))?)),
                    ($($type)?),
                )
            )
            ($($($rem)*)?)
        }
    };
    ($($anything:tt)*) => {
        compile_error!{stringify!($($anything)*)}
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __get_array_length {
    ($e:expr; $len:expr) => ({
        $len
    });
    () => (0);
    ($($e:expr),+ $(,)*) => ({
        [$($crate::__as_unit!($e),)*].len()
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! __length_or_infer {
    (($expr:expr), ($($elem_ty:ty)?), ($(const _)?)) => ({
        let len = $crate::__::Usize;
        if false {
            len.infer_mda $(::<$elem_ty>)? (&$crate::__::ManuallyDrop::new($expr));
        }
        len.get()
    });
    ($expr:tt, $elem_ty:tt, (const $length:expr)) => ( $length );
    ($expr:tt, $elem_ty:tt, (type $type:ty)) => ( <$type as $crate::__::ArrayLength>::LENGTH );
}


#[doc(hidden)]
#[macro_export]
macro_rules! __as_unit {
    ($($tt:tt)*) => { () }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __type_ascription {
    (($e:expr) ()) => { $e };
    (($e:expr) ([$elem_ty:ty; _])) => {
        $crate::__::ArrayAndGhost{
            inner: $e,
            elem_ty: $crate::__::PhantomData::<$elem_ty>,
        }.inner
    };
    (($e:expr) ($ty:ty)) => {
        $crate::__::Identity::<$ty>{inner: $e}.inner 
    };
}





















