//extern crate slack_api;
extern crate iron;
extern crate router;
extern crate alexa;
use iron::prelude::*;
use router::Router;

struct RequestHandler{}
impl alexa::RequestHandler for RequestHandler {
    fn handle_request(&self, req: &alexa::Request) -> alexa::Response {
        match req.body {
            alexa::RequestBody::IntentRequest(ref ir) => {
                match ir.name.as_str() {
                    "DoubleNumber" => {
                        let num_o: Option<f64> = ir.slots.get("num").and_then(|n| n.parse().ok());
                        match num_o {
                            Some(num) => doubled_number_response(num),
                            None => i_dont_understand(),
                        }
                    },
                    _ => i_dont_understand(),
                }
            },
            _ => i_dont_understand(),
        }
    }
}
fn doubled_number_response<'a>(num: f64) -> alexa::Response<'a> {
        alexa::Response {
            session_attributes: None,
            card: None,
            reprompt: None,
            output_speech: Some(alexa::OutputSpeech::Text(format!("Double {} is {}",num,num * 2f64).into())),
            should_end_session: true,
        }
}
fn i_dont_understand<'a>() -> alexa::Response<'a> {
        alexa::Response {
            session_attributes: None,
            card: None,
            reprompt: None,
            output_speech: Some(alexa::OutputSpeech::Text("Oh no, I don't understand what you said!".into())),
            should_end_session: true,
        }
}
fn main() {
    let mut router = Router::new();
    router.get("/healthcheck",handle_healthcheck);
    let rh = RequestHandler{};
    let ih = alexa::IronHandler::new("amzn1.ask.skill.b5e47314-7712-4fbf-aece-3038cbd9a5d4".to_owned(),Box::new(rh));
    router.any("/",ih);
    Iron::new(router).http("0.0.0.0:3000").unwrap();
    //let input = env::args().last().unwrap();
    //let num = input.parse::<f64>().unwrap();
    //println!("{}",num * (2 as f64));
    //let client = hyper::Client::new();
    //println!("{:?}",slack_api::channels::list(&client,"",Some(true)));
    
}
fn handle_healthcheck(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok)))
}
/*
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
*/


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
