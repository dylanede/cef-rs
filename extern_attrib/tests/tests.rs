#![feature(proc_macro)]
//#![feature(custom_attribute)]
extern crate extern_attrib;

#[cfg(test)]
mod tests {
    use extern_attrib::extern_auto;

    #[test]
    fn it_works() {
        #![extern_auto]
        fn decorated() {
            println!("Helloo");
        }
        decorated();
        assert!(true);
    }
}
