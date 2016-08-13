//extern crate slack_api;
extern crate iron;
extern crate bodyparser;
extern crate serde_json;
extern crate router;
use iron::prelude::*;
use serde_json::value::Value;
use std::collections::HashMap;
use router::Router;


include!(concat!(env!("OUT_DIR"), "/response.rs"));

fn main() {
    let mut router = Router::new();
    router.get("/healthcheck",handle_healthcheck);
    router.any("/",handle_request);
    Iron::new(router).http("0.0.0.0:3000").unwrap();
    //let input = env::args().last().unwrap();
    //let num = input.parse::<f64>().unwrap();
    //println!("{}",num * (2 as f64));
    //let client = hyper::Client::new();
    //println!("{:?}",slack_api::channels::list(&client,"",Some(true)));
    
}
fn handle_healthcheck(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "All is well.")))
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
    let mut slots = HashMap::new();
    for (k,v) in m {
        if let &Value::Object(ref o) = v {
            if let Some(sv) = o.get("value") {
                if let &Value::String(ref s) = sv {
                    slots.insert(k.as_str(),s);
                }
            }
        }
    }
    let ir = IntentRequest { name: name, slots: &slots };
    handle_intent_request(&ir)
}
fn handle_intent_request(ir: &IntentRequest) -> IronResult<Response> {
    println!("{:?}",ir);
    match ir.name {
        "DoubleNumber" => handleDNRequest(&ir),
        _ => Ok(Response::with((iron::status::Ok, "Sup dog."))),
    }
}
fn handleDNRequest(ir: &IntentRequest) -> IronResult<Response> {
    let answer = match ir.slots.get("num") {
        Some(ref s) => {
            let num: f64 = s.parse().unwrap();
            let doubled = num * 2.0f64;
            format!("Double {} is {}",num,doubled)
        },
        _ => "I don't understand".to_owned(),
    };
    let response = AlexaResponse::new(&answer);
    let encoded = serde_json::to_string(&response).unwrap();
    Ok(Response::with((iron::status::Ok, encoded)))
}


#[derive(Debug)]
pub struct IntentRequest<'a> {
    name: &'a str,
    slots: &'a HashMap<&'a str, &'a String>,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
