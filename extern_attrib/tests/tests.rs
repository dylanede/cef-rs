#![feature(proc_macro)]
extern crate extern_attrib;

#[cfg(test)]
mod tests {
    use extern_attrib::extern_auto;

    #[test]
    fn extern_auto_resolves_as_expected() {
        #[extern_auto]
        #[cfg(not(extra_attrib_to_make_things_harder = "for the procedural macro"))]
        extern "Rust" fn modified_by_attrib() -> i32 {
            1 + 2
        }

        /// Expect build failure if extern_auto is incorrect.
        #[cfg(target_os = "windows")]
        let f: extern "stdcall" fn() -> i32 = modified_by_attrib;

        /// Expect build failure if extern_auto is incorrect.
        #[cfg(not(target_os = "windows"))]
        let f: extern "C" fn() -> i32 = modified_by_attrib;

        //let i = modified_by_attrib();
        assert!(f() == 3)
    }
}
