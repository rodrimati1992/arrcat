/// Concatenates multiple arrays into one.
///
/// This macro is callable in const contexts.
///
/// [**examples below**](#examples)
///
/// # Syntax
///
/// The syntax of this macro, using `macro_rules!`-like input syntax
///
/// ```text
/// concat_arrays!{
///     $( length_type = $length_type:ty ;)?
///     
///     $( $array_arg:array_expr $(: $argument_type:ty )? ),*
///     $(,)?
/// }
/// ```
///
/// Where `$length_type` is a concrete type.
/// When this argument is passed,
/// a `$length_type::LEN` associated constant
/// is defined with the length of the returned array.
/// [example below](#length-inference-example)
///
/// Where `$array_arg` can be any of:
///
/// - `[ $($array_contents:tt)* ]`: an array literal.
///
/// - A `()`/`{}`-delimited expression of array type. Eg: `(foo.bar())`, `{foo(); bar()}`.
///
/// - `$path:path` expression of array type. Eg: `foo`, `::foo::bar`, `Foo::<T>::BAR`.
///
/// Where `$argument_type` is the type of that argument (always an array).
///
/// ### Special syntax
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
/// Note that due to how `:expr` macro parameters work,
/// arguments of that type from other macros aren't parsed as an array literal,
/// so they may require a type annotation.
///
/// These are two ways that the caller macro can parse that parameter to
/// pass it as an array literal:
/// - `$array_arg:tt`, passed as `$array_arg`
/// - `[$($array_arg:tt)*]`, passed as `[$($array_arg)*]`
///
/// The same limitation applies to parsing type annotations.<br>
/// If the caller macro passes a `$type:ty` as the type of an argument,
/// it'll require the length of the array to be specified.
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
/// <span id = "length-inference-example"></span>
/// ### Length constant
///
/// This macro allows getting the length of the returned array,
/// by declaring an associated constant on a passed-in type.
///
/// The length can then be used anywhere that can access the passed-in type.
///
/// ```rust
/// use arrcat::concat_arrays;
///
/// assert_eq!(foo(0xF00), [0xF00, 3, 5, 8, 13, 21, 34, 55]);
///
/// enum FooLen{}
/// const fn foo(x: u64) -> [u64; FooLen::LEN] {
///     let foo = [x, 3, 5, 8];
///
///     concat_arrays!{
///         // makes the macro define the `FooLen::LEN` associated constant with
///         // the length of the returned array
///         length_type = FooLen;
///         
///         foo: [_; 4],
///         [13, 21, 34, 55],
///     }
/// }
///
/// ```
#[macro_export]
macro_rules! concat_arrays {
    () => ([]);
    ($(length_type = $length_type:ty)?; $($args:tt)* ) => (
        $crate::__concat_arrays_preprocess_inner!{
            (config(length_type($($length_type)?)))
            ($($args)*)
        }
    );
    ( $($args:tt)* ) => (
        $crate::__concat_arrays_inner!{(config(length_type())) ($($args)*)}
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __concat_arrays_preprocess_inner {
    ( $config:tt () ) => {
        $crate::__concat_arrays_inner! {$config ([])}
    };
    ( $config:tt $args:tt ) => {
        $crate::__concat_arrays_inner! {$config $args}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __concat_arrays_inner {
    (
        (
            config(length_type $length_type:tt)

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
                {$crate::__declare_length_type_and_pass!(
                    $length_type,
                    {
                        let mut len = 0;
                        $( len += $len; )*
                        len
                    }
                )}
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
        $crate::__concat_arrays_inner!{
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
        $crate::__concat_arrays_inner!{
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
        $crate::__concat_arrays_inner!{
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
        $crate::__concat_arrays_inner!{
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
        $crate::__concat_arrays_inner!{
            $prev
            (($expr) $(: $($rem)*)?)
        }
    };

    ( $prev:tt ( $expr:path $(, $($rem:tt)*)? ) ) => {
        $crate::__concat_arrays_inner!{
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

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_length_type_and_pass {
    (($length_type:ty), $length:expr) => {{
        impl $length_type {
            pub const LEN: $crate::__::usize = $length;
        }

        <$length_type>::LEN
    }};
    ((), $length:expr) => {
        $length
    };
}
