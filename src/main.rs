extern crate slack_api;
extern crate iron;
extern crate router;
extern crate alexa;
extern crate hyper;
use iron::prelude::*;
use slack_api::*;
use router::Router;
use std::collections::BTreeMap;
use std::borrow::Cow;

static API_KEY: &'static str = "";

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
                    "SetReminder" => {
                        set_reminder(&ir.slots)
                    },
                    "ReadUnread" => {
                        read_unread()
                    },
                    _ => i_dont_understand(),
                }
            },
            _ => i_dont_understand(),
        }
    }
}

fn read_unread<'a>() -> alexa::Response<'a> {
    let res = slack_api::rtm::start(&get_client(),API_KEY,Some(false),Some(false));
    let channels = res.unwrap().channels;
    let unread_channels = channels.iter().filter(|c| if let Some(c) = c.unread_count_display {c != 0 } else {false} ).collect::<Vec<_>>();
    if unread_channels.len() == 0 {
        respond_with_text("You don't have any unread messages right now".into())
    } else {
        respond_with_text(unread_channels.iter().map(talk_through_channel).fold("".to_owned(),|memo, s| memo + &s).into())
    }
}

fn talk_through_channel(c: &&Channel) -> String {
    let text = format!("{} unread messages for channel {}.  ",c.unread_count_display.unwrap(),c.name);
    let last_read = c.last_read.as_ref().map(|s| s.as_str());
    let messages = slack_api::channels::history(&get_client(),API_KEY,&c.id,None,last_read,None,None).unwrap().messages;
    messages.iter().filter_map(talk_through_message).fold(text, |memo,s| memo + &s)
}

fn talk_through_message(m: &Message) -> Option<String> {
    match m {
        &Message::Standard { ref user, ref text , ..} => { Some(format!("{} says {}.  ",user.as_ref().unwrap(),text.as_ref().unwrap())) },
        &Message::BotMessage { ref username, ref text , ..} => { Some(format!("{} says {}.  ",username.as_ref().map(|s| s.as_str()).unwrap_or("Unknown bot"),text.as_ref().unwrap())) },
        _ => None,
    }
}

fn set_reminder<'a>(slots: &BTreeMap<String,String>) -> alexa::Response<'a> {
    let at_time = match slots.get("at_time"){
        Some(t) => t,
        _ => { return i_dont_understand(); }
    };
    let reminder = match slots.get("reminder"){
        Some(r) => r,
        _ => { return i_dont_understand(); }
    };
    let res = slack_api::reminders::add(&get_client(),API_KEY,reminder,at_time,None);
    println!("{:?}",res);
    if res.is_ok() {
        respond_with_text(format!("Ok, I set a reminder to {} for {}",reminder,at_time).into())
    } else {
        respond_with_text("Oh no, something went wrong!".into())
    }
}
fn respond_with_text(txt: Cow<str>) -> alexa::Response {
        alexa::Response {
            session_attributes: None,
            card: None,
            reprompt: None,
            output_speech: Some(alexa::OutputSpeech::Text(txt)),
            should_end_session: true,
        }
}
fn get_client() -> hyper::client::Client {
    hyper::client::Client::new()
}
fn doubled_number_response<'a>(num: f64) -> alexa::Response<'a> {
        respond_with_text(format!("Double {} is {}",num,num * 2f64).into())
}
fn i_dont_understand<'a>() -> alexa::Response<'a> {
    respond_with_text("Oh no, I don't understand what you said!".into())
}
fn main() {
    let mut router = Router::new();
    router.get("/healthcheck",handle_healthcheck);
    let rh = RequestHandler{};
    let ih = alexa::IronHandler::new("amzn1.ask.skill.b5e47314-7712-4fbf-aece-3038cbd9a5d4".to_owned(),Box::new(rh));
    router.any("/",ih);
    Iron::new(router).http("0.0.0.0:3000").unwrap();
    
}
fn handle_healthcheck(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok)))
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
