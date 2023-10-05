use std::fs;

use postdude_types::request::Request;
use postdude_lib;

fn main() {
    let file = "./get-data.json";
    let text = fs::read_to_string(file).expect("Can not find file");
    let req: Request = Request::get_request(text.as_str());
    let resp = postdude_lib::get_data(&req);

    println!("{:?}", req);
}
