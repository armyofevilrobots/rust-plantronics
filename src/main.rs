#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate reqwest;
use clap::{App, Arg};
use serde_json::Value;
use std::thread::sleep;
use std::time::Duration;

mod encoding;
type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn get_session_id(name: &String) -> Result<String> {
    let request_url = format!(
        "http://localhost:32017/Spokes/SessionManager/Register?name={name}",
        name = name
    );
    println!("{}", request_url);
    let mut response = reqwest::get(&request_url)?;
    let out: encoding::AppRegistration = response.json()?;
    println!("{:?}", out);
    let request_url = format!("http://localhost:32017/Spokes/DeviceServices/Attach?uid=0123456789");
    let mut response = reqwest::get(&request_url)?;
    let out: encoding::PlantronicsResponse = response.json()?;
    println!("{:?}", out);
    if !out.is_error && out.type_name == "SessionHash" {
        let session_id = match out.result {
            Value::String(outstr) => outstr,
            _ => "".to_string(),
        };
        if session_id == "" {
            return Err("No session ID in the result.".into());
        } else {
            return Ok(session_id);
        }
    } else {
        return Err("Undefined error from session endpoint.".into());
    }
}

fn get_dse_from_json(item: &Value) -> Result<encoding::DataServiceEvent> {
    let json_str = serde_json::to_string(item)?;
    println!("{:?}", &json_str);
    let ev: encoding::DataServiceEvent = serde_json::from_str(&json_str)?;
    return Ok(ev);
}

fn get_events(session_id: &String) -> Result<Vec<encoding::DataServiceEvent>> {
    // Wherein we http://localhost:32017/Spokes/DeviceServices/Events?sess=$sess
    let request_url = format!(
        "http://localhost:32017/Spokes/DeviceServices/Events?sess={sess}",
        sess = session_id
    );
    let mut response = reqwest::get(&request_url)?;
    let out: encoding::PlantronicsResponse = response.json()?;
    println!("{:?}", out);
    if !out.is_error && out.type_name == "DeviceEventArray" {
        let mut eventresult: Vec<encoding::DataServiceEvent> = Vec::new();
        match out.result {
            Value::Array(outvec) => {
                for item_result in outvec.into_iter() {
                    println!("item_result is {:?}", item_result);
                    eventresult.push(get_dse_from_json(&item_result).unwrap());
                }
            }
            _ => return Err("Unexpected Result type from DeviceEventArray.".into()),
        };
        return Ok(eventresult);
    } else {
        return Err("Undefined error from session endpoint.".into());
    }
}

fn main() {
    let _matches = App::new("rust-plantronics")
        .version("0.0.1")
        .author("Derek Anderson <derek@armyofevilrobots.com>")
        .about("Monitors state of a plantronics headset and sends events to various endpoints.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .takes_value(true)
                .help("The BaseURL of the plantronics API (http://localhost:32017/)"),
        )
        .get_matches();

    let token = match get_session_id(&"rust-plantronics".to_string()) {
        Ok(token) => token,
        Err(e) => {
            println!("Unable to retrieve token due to err: {}", e);
            return;
        }
    };
    println!("{:?}", token);
    // Poll loop, forever...
    let delay_time = Duration::from_secs(1);
    loop {
        let events = get_events(&token);
        //println!("{}", );

        for event in events.into_iter() {
            println!("ev: {:?}", event);
        }
        sleep(delay_time);
    }
}
