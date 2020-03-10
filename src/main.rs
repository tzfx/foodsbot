use std::{env, process, error};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use slack;
use slack::{Event, RtmClient};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Delivery {
    RestaurantName: String,
    Cutoff: String,
    Dropoff: String
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Day {
    Day: String,
    Deliveries: Vec<Delivery>
}

// Defaults.
const LOCATION: &str = "10378"; // NBP308
const FOODSBY_CHANNEL: &str = "bot-testing";

struct SlackHandler {
    payload: Vec<String>,
    channel: String,
}

#[allow(unused_variables)]
impl slack::EventHandler for SlackHandler {

    fn on_close(&mut self, cli: &RtmClient) {
        println!("Connection to slack closed.");
    }

    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("Connection to slack open.");
        let foodsby_channel_id = cli.start_response()
            .channels
            .as_ref()
            .and_then(|channels| {
                          channels
                              .iter()
                              .find(|chan| match chan.name {
                                        None => false,
                                        Some(ref name) => String::from(name).eq(&self.channel),
                                    })
                      })
            .and_then(|chan| chan.id.as_ref())
            .expect("foodsby channel not found");
        for message in self.payload.iter() {
            let _ = cli.sender().send_message(&foodsby_channel_id, message);
        }

        let _ = cli.sender().shutdown();
        
    }
}

fn create_payload(day: &Day) -> Result<Vec<String>, Box<dyn error::Error>> {
    let mut payload = vec![String::from(":robot_face: Today's Foodsby choices are as follows:")];
    for delivery in day.Deliveries.iter() {
        payload.push(format!("{}: order by {}, delivery at {}", &delivery.RestaurantName, &delivery.Cutoff, &delivery.Dropoff));
    }
    Ok(payload)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let user_token = env::var("FOODSBOT_SLACK_TOKEN");
    if user_token.is_err() {
        eprintln!("No FOODSBOT_SLACK_TOKEN was set.");
        process::exit(1);
    }

    let location = env::var("FOODSBOT_LOCATION").unwrap_or(String::from(LOCATION));
    let channel = env::var("FOODSBOT_CHANNEL").unwrap_or(String::from(FOODSBY_CHANNEL));
    let foodsby_api: String = format!("https://www.foodsby.com/api-monolith/location/{}/schedule?day={}&duration=1", location, Local::now().format("%Y%m%d"));

    let days = reqwest::blocking::get(&foodsby_api)?
        .json::<Vec<Day>>()?;
        
    println!("Retrieved daily foodsby schedule.");
    // println!("{:#?}", days);
    println!("Sending message to slack!");
    let mut handler = SlackHandler {
        payload: create_payload(&days[0]).unwrap(),
        channel: channel
    };
    let connect_and_send = RtmClient::login_and_run(&user_token.unwrap(), &mut handler);
    match connect_and_send {
        Ok(_) => Ok(()),
        Err(err) => panic!("Failed to send daily foodsby info to slack. {}", err)
    }
}