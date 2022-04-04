/// Concatenates multiple arrays into one.
///
/// This macro is callable in const contexts.
///
/// [**examples below**](#examples)
///
/// # Special syntax
///
/// This macro allows inferring the length of arrays in type annotations, with a `_` length:
/// ```rust
/// # use arrcat::concat_arrays;
/// # assert_eq!(
/// // The `_` tells this macro to infer the length.
/// concat_arrays!([3, 5, 8]: [u8; _])
/// #   [..],
/// #   [3, 5, 8]
/// # );
/// ```
///
/// # Limitations
///
/// This macro cannot concatenate arrays whose length depends on a
/// surrounding generic parameter.
///
/// These are the only kinds of arguments that don't require a type annotation:
///
/// - Constants of fully inferred type.
///
/// - Array literals (they can contain runtime values).
///
/// ### Argument from other macros
///
/// Note that due to how `$expr_arg:expr` macro parameters work,
/// `$expr_arg` arguments from other macros aren't parsed as an array literal,
/// so they may require a type annotation.
///
/// These are two ways that the caller macro can parse that parameter to
/// pass it as an array literal:
/// - `$array_arg:tt`, passed as `$array_arg`
/// - `[$($array_arg:tt)*]`, passed as `[$($array_arg)*]`
///
/// The same limitation applies to parsing type annotations.
/// If the caller macro passes a `$type:ty` as the type of an argument,
/// it'll be treated like a concrete type,
/// and the `_` length argument (in `[T; _]`) won't be parsed by this macro.
///
/// # Examples
///
/// ### Basic
///
/// ```rust
/// use arrcat::concat_arrays;
///
/// assert_eq!(
///     concat_arrays!(["foo", "bar"], ["baz"; 3]),
///     ["foo", "bar", "baz", "baz", "baz"],
/// );
/// ```
///
/// ### Constant arguments
///
/// ```rust
/// use arrcat::concat_arrays;
///
/// assert_eq!(consts(), [Some(3), Some(5), None, Some(8), None, None, None]);
///
///
/// fn consts() -> [Option<u16>; 7] {
///     concat_arrays!(
///         [Some(3), Some(5)],
///         // `const`ants can passed unparenthesized.
///         FOO,
///         // Constant array args must have a fully inferred type.
///         // Since `Bar::ARR` isn't fully inferred, it requires a type annotation.
///         Bar::ARR: [Option<u16>; _],
///     )
/// }
///
/// const FOO: [Option<u16>; 2] = [None, Some(8)];
///
/// struct Bar<T>(T);
///
/// impl<T> Bar<T> {
///     pub const ARR: [Option<T>; 3] = [None, None, None];
/// }
///
/// ```
///
/// ### Non-constant arguments
///
/// ```rust
/// use arrcat::concat_arrays;
///
/// assert_eq!(VARS, [12, 13, 14, 24, 28, 32, 243]);
///
///
/// const VARS: [u16; 7] = {
///     let twelve = 12;
///     let variable = [24, 28];
///
///     concat_arrays!(
///         [twelve, twelve + 1, twelve + 2],
///         // Array variables can passed unparenthesized, and require type annotations.
///         // Non-constant arguments require type annotation
///         // for this macro to know their array length.
///         variable: [_; 2],
///         // Other expressions need to be parenthesized.
///         (make_array(5)): [_; 2],
///     )
/// };
///
/// const fn make_array(x: u32) -> [u16; 2] {
///     [2u16.pow(x), 3u16.pow(x)]
/// }
/// ```
///
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

    ( $prev:tt ( $expr:path $(: $($rem:tt)*)? ) ) => {
        $crate::__concat_arrays_preprocess_inner!{
            $prev
            (($expr) $(: $($rem)*)?)
        }
    };

    ( $prev:tt ( $expr:path $(, $($rem:tt)*)? ) ) => {
        $crate::__concat_arrays_preprocess_inner!{
            $prev
            (($expr) $(, $($rem)*)?)
        }
    };

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
    ($($tt:tt)*) => {
        ()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __type_ascription {
    (($e:expr) ()) => {
        $e
    };
    (($e:expr) ([$elem_ty:ty; _])) => {
        $crate::__::ArrayAndGhost {
            inner: $e,
            elem_ty: $crate::__::PhantomData::<$elem_ty>,
        }
        .inner
    };
    (($e:expr) ($ty:ty)) => {
        $crate::__::Identity::<$ty> { inner: $e }.inner
    };
}
