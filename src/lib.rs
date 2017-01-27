extern crate hyper;
extern crate regex;
#[macro_use]
extern crate ruru;
extern crate hyper_native_tls;

use std::io::Read;
use std::error::Error;

use hyper::Client;
use hyper::Url;
use hyper::header::UserAgent;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use regex::Regex;
use ruru::{Class, Object, RString, Array};

class!(Speller);

methods!(
    Speller,
    itself,

    fn query(term: RString) -> Array {
        let mut results = Array::with_capacity(2);

        let string_term = match term {
            Ok(t) => { t.to_string() },
            _ => { return Array::new() }
        };

        match get_google_suggestion(&string_term) {
            Some(result) => {
                results.push(RString::new(&result));
            },
            _ => {}
        }
        match get_ddg_suggestion(&string_term) {
            Some(result) => {
                results.push(RString::new(&result));
            },
            _ => {}
        }
        return results;
    }
);

#[no_mangle]
pub extern fn initialize_speller() {
    Class::new("Speller", None).define(|itself| {
        itself.def("query", query);
    });
}

pub fn get_google_suggestion(term: &String) -> Option<String> {
    let url = format!("https://www.google.com/search?q={}", term);

    let response = get_content(&url);

    let re = Regex::new(r"class=.spell. \S+><b><i>(?P<word>[^<]+)").unwrap();
    match re.captures(&response) {
        Some(caps) => {
            return Some(String::from(&caps["word"]))
        },
        None => {
            return None
        }
    }
}

pub fn get_ddg_suggestion(term: &String) -> Option<String> {
    let url = format!("https://duckduckgo.com/html/?q={}", term);

    let response = get_content(&url);

    let re = Regex::new(r"Including results for <a[^>]+><b>(?P<word>[^<]+)").unwrap();
    match re.captures(&response) {
        Some(caps) => {
            return Some(String::from(&caps["word"]))
        },
        None => {
            return None
        }
    }
}

fn get_content(url: &str) -> String {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let ua = UserAgent(String::from("Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:50.0) Gecko/20100101 Firefox/50.0"));

    let url = Url::parse(url).unwrap();

    match client.get(url.as_str()).header(ua).send() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn google_example() {
		let result = get_google_suggestion(&String::from("mistakee"));
		assert_eq!(result.unwrap(), String::from("mistake"));
    }

    #[test]
    fn ddg_example() {
		let result = get_ddg_suggestion(&String::from("errorr"));
		assert_eq!(result.unwrap(), String::from("error"));
    }
}
