#[macro_use]
extern crate ruru;

use ruru::{Class, Object, RString};

class!(Thing);

methods!(
    Thing,
    itself,

    fn say_hello() -> RString {
        let str = String::from("hello");
        RString::new(&str)
    }
);

#[no_mangle]
pub extern fn initialize_thing() {
    Class::new("Thing", None).define(|itself| {
        itself.def("say_hello", say_hello);
    });
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
