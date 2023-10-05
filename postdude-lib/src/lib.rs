use std::io::{stdout, Write};
use curl::easy::Easy;

use postdude_types::request::Request;

pub fn get_data(request: &Request) -> Result<u32, curl::Error> {
    let mut easy = Easy::new();
    easy.url(request.url).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.response_code()
}
