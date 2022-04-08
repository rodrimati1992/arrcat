use crate::concat_arrays;

use core::{cmp::PartialEq, fmt::Debug};

macro_rules! generic_test {
    ($conv:ident => $($code:tt)*) => {
        fn inside<T>()
        where
            T: Copy + From<u16> + PartialEq + Debug
        {
            let $conv = <T as From<u16>>::from;
            $($code)*
        }

        inside::<u16>();
    };
}

#[test]
fn test_empty_array() {
    generic_test! {_f =>
        {
            asserteq(concat_arrays!(), [0u8; 0]);
        }
        {
            asserteq(concat_arrays!([]), [0u8; 0]);
        }
        {
            let arr = concat_arrays!([]: [T; 0]);
            asserteq(arr, []);
        }
        {
            let arr = concat_arrays!([]: [T; _]);
            asserteq(arr, []);
        }
    }
}

#[test]
fn test_array_arg() {
    generic_test! {f=>
        // elems
        {
            let arr = concat_arrays!([f(3), f(4)]);
            asserteq(arr, [f(3), f(4)]);
        }
        // repeated, transparent type ascription
        {
            let arr = concat_arrays!([f(3); 2]: [T; 2]);
            asserteq(arr, [f(3), f(3)]);
        }
        // repeated, transparent type ascription, inferred length
        {
            let arr = concat_arrays!([f(3); 2]: [T; _]);
            asserteq(arr, [f(3), f(3)]);
        }
        // repeated, opaque type ascription
        {
            let arr = concat_arrays!([f(3); 2]: ([T; 2]));
            asserteq(arr, [f(3), f(3)]);
        }
    }
}

#[test]
fn test_const_arg() {
    // constant
    {
        const B: D = D(10);
        const C: [D; 2] = [B; 2];
        let arr = concat_arrays!(C);
        asserteq(arr, [D(10), D(10)]);
    }
    // constant, transparent type ascription
    asserteq(concat_arrays!(FooConst::C: [u8; 4]), [3, 3, 3, 3]);
    asserteq(concat_arrays!((FooConst::C): [u8; 4]), [3, 3, 3, 3]);

    // constant, transparent type ascription, inferred length
    asserteq(concat_arrays!(FixedLengthArray::C: [u32; _]), [5, 5, 5]);
    asserteq(concat_arrays!((FixedLengthArray::C): [u32; _]), [5, 5, 5]);

    // constant, opaque type ascription
    {
        type T = [u16; 2];
        let arr = concat_arrays!((FooConst::C): T);
        asserteq(arr, [4, 4]);
    }
}

#[test]
fn test_runtime_variable_arg() {
    // variable, transparent type ascription
    {
        let var = FooConst::C;
        let arr = concat_arrays!(var: [u8; 4]);
        asserteq(arr, [3, 3, 3, 3]);
    }
    // variable, opaque type ascription
    {
        type T = [u16; 2];
        let var = FooConst::C;
        let arr = concat_arrays!(var: T);
        asserteq(arr, [4, 4]);
    }

    // variable in parens, transparent type ascription
    {
        let var = FooConst::C;
        let arr = concat_arrays!((var): [u8; 4]);
        asserteq(arr, [3, 3, 3, 3]);
    }
    // variable in parens, opaque type ascription
    {
        type T = [u16; 2];
        let var = FooConst::C;
        let arr = concat_arrays!((var): T);
        asserteq(arr, [4, 4]);
    }
}

#[test]
fn test_runtime_expression_arg() {
    fn do_nothing() {}
    fn rt<T>(x: T) -> T {
        x
    }

    // runtime expression, transparent type ascription, expr in braces
    {
        let arr = concat_arrays!(
            {
                do_nothing();
                rt(FooConst::C)
            }: [u8; 4]
        );
        asserteq(arr, [3, 3, 3, 3]);
    }
    // runtime expression, transparent type ascription, expr in parentheses braces
    {
        let arr = concat_arrays!((rt(FooConst::C)): [u8; 4]);
        asserteq(arr, [3, 3, 3, 3]);
    }
    // runtime expression, opaque type ascription
    {
        type T = [u16; 2];
        let arr = concat_arrays!((rt(FooConst::C)): T);
        asserteq(arr, [4, 4]);
    }
}

#[test]
fn test_path_args() {
    {
        let arr = concat_arrays!(::core_extensions::ConstDefault::DEFAULT: [u8; 3]);
        asserteq(arr, [0, 0, 0]);
    }
    {
        let arr = concat_arrays!(core_extensions::ConstDefault::DEFAULT: [Option<Str>; 3]);
        asserteq(arr, [None, None, None]);
    }
}

mod arrays {
    pub const ARR0: [u16; 3] = [16, 32, 64];
}

#[test]
fn test_many_args() {
    {
        const C: [u16; 3] = [21, 34, 55];

        let var = [89, 144, 233];

        let arr = concat_arrays!([3, 5, 8], [13; 2], self::arrays::ARR0, (C), (var): [_; 3],);
        asserteq(arr, [3, 5, 8, 13, 13, 16, 32, 64, 21, 34, 55, 89, 144, 233]);
    }
}

/// Tests all kinds of arguments with:
/// - interspersed commas
/// - trailing commas
#[test]
fn test_comma_sep() {
    fn rt<T>(x: T) -> T {
        x
    }

    const RHS: [u16; 5] = [3, 5, 8, 13, 13];
    type Arr2 = [u16; 2];
    type Arr3 = [u16; 3];

    // arrays, no type ascription
    asserteq(concat_arrays!([3, 5, 8], [13, 13]), RHS);
    asserteq(concat_arrays!([3, 5, 8], [13; 2],), RHS);
    asserteq(concat_arrays!([3, 5, 8], [13; 2], []), RHS);

    // arrays, with transparent type ascription
    asserteq(concat_arrays!([3, 5, 8]: [_; 3], [13; 2]: [_; 2]), RHS);
    asserteq(concat_arrays!([3, 5, 8]: [_; 3], [13, 13]: [_; 2],), RHS);
    asserteq(concat_arrays!([3, 5, 8]: [_; 3], [13, 13]: [_; 2], []), RHS);

    // arrays, with opaque type ascription
    asserteq(concat_arrays!([3, 5, 8]: Arr3, [13, 13]: Arr2), RHS);
    asserteq(concat_arrays!([3, 5, 8]: Arr3, [13; 2]: Arr2,), RHS);
    asserteq(concat_arrays!([3, 5, 8]: Arr3, [13; 2]: Arr2, []), RHS);

    const B: [u16; 2] = [13, 13];
    // expression, no type ascription
    asserteq(concat_arrays!([3, 5, 8], (B)), RHS);
    asserteq(concat_arrays!([3, 5, 8], (B),), RHS);
    asserteq(concat_arrays!([3, 5, 8], B), RHS);
    asserteq(concat_arrays!([3, 5, 8], B,), RHS);
    asserteq(concat_arrays!([3, 5, 8], B, []), RHS);

    // expression, with transparent type ascription
    asserteq(concat_arrays!([3, 5, 8], (rt(B)): [u16; 2]), RHS);
    asserteq(concat_arrays!([3, 5, 8], (rt(B)): [u16; 2],), RHS);
    asserteq(concat_arrays!([3, 5, 8], B: [u16; 2]), RHS);
    asserteq(concat_arrays!([3, 5, 8], B: [u16; 2],), RHS);
    asserteq(concat_arrays!([3, 5, 8], B: [u16; 2], []), RHS);

    // expression, with opaque type ascription
    asserteq(concat_arrays!([3, 5, 8], (rt(B)): Arr2), RHS);
    asserteq(concat_arrays!([3, 5, 8], (rt(B)): Arr2,), RHS);
    asserteq(concat_arrays!([3, 5, 8], B: Arr2), RHS);
    asserteq(concat_arrays!([3, 5, 8], B: Arr2,), RHS);
    asserteq(concat_arrays!([3, 5, 8], B: Arr2, []), RHS);
}

#[test]
fn test_macro_called_by_macro() {
    {
        macro_rules! prepended_lit {
            ($prefix:tt) => {
                concat_arrays!($prefix, ["hello", "world"])
            };
        }

        // parsed as array by macro
        asserteq(
            prepended_lit!(["foo", "bar"]),
            ["foo", "bar", "hello", "world"],
        );
        asserteq(prepended_lit!(["foo"; 2]), ["foo", "foo", "hello", "world"]);

        // constant, not parsed as array by macro
        asserteq(
            prepended_lit!((["foo", "bar"])),
            ["foo", "bar", "hello", "world"],
        );
        asserteq(
            prepended_lit!((["foo"; 2])),
            ["foo", "foo", "hello", "world"],
        );
    }

    {
        let array = [3, 5, 8];

        macro_rules! prepended {
            ($prefix:tt : $ty:tt) => {
                concat_arrays!($prefix: $ty, [80, 81])
            };
        }

        asserteq(prepended!([3, 5, 6]: _), [3, 5, 6, 80, 81]);
        asserteq(prepended!([3, 5, 6]: [_; _]), [3, 5, 6, 80, 81]);
        asserteq(prepended!(array: [_; 3]), [3, 5, 8, 80, 81]);
    }
}

#[test]
fn length_type_arg() {
    {
        enum L {}
        let _: &[u8] = &concat_arrays!(length_type = L;);
        assert_eq!(L::LEN, 0);
    }
    {
        enum L {}
        let _: &[u8] = &concat_arrays!(length_type = L; [1]);
        assert_eq!(L::LEN, 1);
    }
    {
        const C: [u8; 5] = [5, 8, 13, 21, 34];
        enum L {}
        const A: [u8; L::LEN] = concat_arrays!(length_type = L; [1], [2; 3], C);
        asserteq(A, [1, 2, 2, 2, 5, 8, 13, 21, 34]);
        assert_eq!(L::LEN, 9);
    }
}

#[derive(Debug, PartialEq)]
struct Str(&'static str);

impl ::core_extensions::ConstDefault for Str {
    const DEFAULT: Self = Self("");
}

impl Drop for Str {
    fn drop(&mut self) {}
}

trait ConstVal<T> {
    const C: T;
}

struct FooConst;

impl ConstVal<u8> for FooConst {
    const C: u8 = 3;
}

impl ConstVal<u16> for FooConst {
    const C: u16 = 4;
}

impl ConstVal<u32> for FooConst {
    const C: u32 = 5;
}

impl<T, const L: usize> ConstVal<[T; L]> for FooConst
where
    Self: ConstVal<T>,
{
    const C: [T; L] = [<FooConst as ConstVal<T>>::C; L];
}

struct FixedLengthArray;

impl<T> ConstVal<[T; 3]> for FixedLengthArray
where
    FooConst: ConstVal<T>,
{
    const C: [T; 3] = [<FooConst as ConstVal<T>>::C; 3];
}

#[derive(Debug, PartialEq)]
struct D(u32);

impl Drop for D {
    fn drop(&mut self) {}
}

#[track_caller]
fn asserteq<T, const L: usize, const R: usize>(found: [T; L], expected: [T; R])
where
    T: PartialEq + Debug,
{
    assert_eq!(&found[..], &expected[..]);
}
