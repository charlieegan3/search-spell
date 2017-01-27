extern crate hyper;
#[macro_use]
extern crate ruru;
extern crate hyper_native_tls;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use ruru::{Class, Object, RString};

use std::io::Read;
use std::error::Error;

fn get_content(url: &str) -> String {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    match client.get(url).send() {
        Ok(mut response) => {
            let mut buffer = String::new();
            response.read_to_string(&mut buffer);
            return buffer;
        },
        Err(err) => {
            return String::from(err.description());
        }
    }
}

class!(Thing);

methods!(
    Thing,
    itself,

    fn say_hello(url: RString) -> RString {
        match url {
            Ok(url_string) => {
                let response = get_content(&url_string.to_string());
                return RString::new(&response);
            },
            Err(err) => {
                return RString::new(err.description());
            }
        }
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
