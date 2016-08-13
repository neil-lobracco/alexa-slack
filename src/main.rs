//extern crate slack_api;
extern crate iron;
extern crate bodyparser;
extern crate serde_json;
use std::env;
use iron::prelude::*;
use serde_json::value::Value;

fn main() {
    let chain = Chain::new(handle_request);
    Iron::new(chain).http("localhost:3000").unwrap();
    //let input = env::args().last().unwrap();
    //let num = input.parse::<f64>().unwrap();
    //println!("{}",num * (2 as f64));
    //let client = hyper::Client::new();
    //println!("{:?}",slack_api::channels::list(&client,"",Some(true)));
    
}
fn handle_request(req: &mut Request) -> IronResult<Response> {
    let json_body = req.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => handle_json(&json_body),
        Ok(None) => { println!("No body"); bad_request()},
        Err(err) => { println!("Error: {:?}", err); bad_request() },
    }
}

fn bad_request() -> IronResult<Response> {
    Ok(Response::with((iron::status::BadRequest, "Bad Request")))
}


fn handle_json(json: &serde_json::value::Value) -> IronResult<Response> {
    match json {
        &serde_json::value::Value::Object(ref m) => handle_object(m),
        _  => bad_request(),
    }
}

fn handle_object(m: &serde_json::value::Map<String,serde_json::value::Value>) -> IronResult<Response> {
    if let Some(r) = m.get("request") {
        if let &Value::Object(ref ro) = r {
            return match ro.get("type") {
                Some(t) => {
                    match t {
                        &serde_json::value::Value::String(ref s) => {
                            if s == "IntentRequest" {
                                if let Some(i) = ro.get("intent"){
                                    if let &Value::Object(ref io) = i {
                                        handle_intent_object(io)
                                    } else {
                                        bad_request()
                                    }
                                }
                                else {
                                    bad_request()
                                }
                            }
                            else {
                                bad_request()
                            }
                        },
                        _ => bad_request(),
                    }
                },
                None => bad_request(),
            }
        }
    }
    bad_request()
}

fn handle_intent_object(m: &serde_json::value::Map<String,Value>) -> IronResult<Response> {
    println!("{:?}",m);
    match m.get("name") {
        Some(i) => {
            match i {
                &Value::String(ref name) => {
                    match m.get("slots") {
                        Some(s) => {
                            match s {
                                &Value::Object(ref so) => {
                                    handle_intent_request_object(name, so)
                                },
                                _ => bad_request(),
                            }
                        },
                        _ => bad_request(),
                    }
                },
                _ => bad_request(),
            }
        },
        _ => bad_request(),
    }
}

fn handle_intent_request_object(name: &str, m: &serde_json::value::Map<String,Value>) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Sup dog.")))
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
