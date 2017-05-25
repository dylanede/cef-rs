//! Macro to choose ABI based on target_os. Thanks to
//! https://github.com/zoomulator for the original macro!
//!
//! Consider replacing this with crate extern_attrib when proc_macro_attribute
//! lands in stable.

//#![macro_export]

//#[cfg(not(target_os="windows"))]
macro_rules! extern_auto_fn(
    ($name:ident <$($gen:ident : $traitbound:ident),*> ($($argn:ident : $argt:ty),*) -> $ret:ty $block:block) => (
        extern "C" fn $name <$($gen : $traitbound),*>($($argn : $argt),*) -> $ret $block
    );
    ($name:ident <$($gen:ident : $traitbound:ident),*> ($($argn:ident : $argt:ty),*) $block:block) => (
        extern "C" fn $name <$($gen : $traitbound),*>($($argn : $argt),*) $block
    );
    (($($argn:ident : $argt:ty),*) -> $ret:ty) => (
        extern "C" fn($($argn : $argt),*) -> $ret
    );
    ($name:ident <$($gen:ident),*> ($($argn:ident : $argt:ty),*) -> $ret:ty $block:block) => (
        extern "C" fn $name <$($gen),*>($($argn : $argt),*) -> $ret $block
    );
    ($name:ident <$($gen:ident),*> ($($argn:ident : $argt:ty),*) $block:block) => (
        extern "C" fn $name <$($gen),*>($($argn : $argt),*) $block
    );
    ($name:ident ($($argn:ident : $argt:ty),*) -> $ret:ty $block:block) => (
        extern "C" fn $name($($argn : $argt),*) -> $ret $block
    );
    ($name:ident ($($argn:ident : $argt:ty),*) $block:block) => (
        extern "C" fn $name($($argn : $argt),*) $block
    );
);

#[test]
fn extern_auto_fn_expands_as_expected() {
    extern_auto_fn!(modified_by_attrib(i: i32) -> i32 {
        i * i
    });
    /*
    extern_auto_fn!(modified_by_attrib2<T>(i: T) -> T {
        i * i
    });
    */

    /// Should cause build error if extern_auto_fn is incorrect.
    #[cfg(target_os = "windows")]
    let f1: extern "stdcall" fn(i: i32) -> i32 = modified_by_attrib;
    //let f2: extern "stdcall" fn(i: i32) -> i32 = modified_by_attrib2::<i32>;

    /// Should cause build error if extern_auto_fn is incorrect.
    #[cfg(not(target_os = "windows"))]
    let f1: extern "C" fn(i: i32) -> i32 = modified_by_attrib;
    //let f2: extern "C" fn(i: i32) -> i32 = modified_by_attrib2::<i32>;

    assert!(f1(123) == 123 * 123);
    //assert!(f2(123) == 123 * 123);
}
