//extern crate hyper;
//extern crate slack_api;
use std::env;

fn main() {
    let input = env::args().last().unwrap();
    let num = input.parse::<f64>().unwrap();
    println!("{}",num * (2 as f64));
    //let client = hyper::Client::new();
    //println!("{:?}",slack_api::channels::list(&client,"",Some(true)));
    
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
