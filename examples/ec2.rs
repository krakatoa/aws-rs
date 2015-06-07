extern crate aws;

#[macro_use]
extern crate log;
extern crate env_logger;
use aws::request::ApiClient;
use aws::credentials::Credentials;
use std::io::Read;

pub fn main() {
    env_logger::init().unwrap();
    let cred = Credentials::create("YOUR-ACCESS-KEY-ID", "YOUR-SECRET-ACCESS-KEY");
    let region = "us-east-1";
    let service = "ec2";

    let client = ApiClient::new(cred, region, service);
    let res = client.get("DescribeInstances");
    let mut output = String::new();
    res.unwrap().read_to_string(&mut output);
    println!("{:?}", output)
}
