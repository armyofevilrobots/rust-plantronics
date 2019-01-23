#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;

// use reqwest;
use serde_json::Value;
use std::thread::sleep;
use std::time::Duration;

mod config;
mod encoding;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn get_session_id(name: &String) -> Result<String> {
    let request_url = format!(
        "{baseurl}Spokes/SessionManager/Register?name={name}",
        baseurl = config::CONFIG
            .value_of("url")
            .unwrap_or(config::DEFAULT_URL),
        name = name,
    );
    println!("{}", request_url);
    let mut response = reqwest::get(&request_url)?;
    let out: encoding::AppRegistration = response.json()?;
    println!("{:?}", out);
    let request_url = format!(
        "{baseurl}Spokes/DeviceServices/Attach?uid={app_uid}",
        baseurl = config::CONFIG
            .value_of("url")
            .unwrap_or(config::DEFAULT_URL),
        app_uid = config::APP_UID,
    );
    let mut response = reqwest::get(&request_url)?;
    let out: encoding::PlantronicsResponse = response.json()?;
    // println!("{:?}", out);
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
    // println!("{:?}", &json_str);
    let ev: encoding::DataServiceEvent = serde_json::from_str(&json_str)?;
    return Ok(ev);
}

fn get_events(session_id: &String) -> Result<Vec<encoding::DataServiceEvent>> {
    // Wherein we http://localhost:32017/Spokes/DeviceServices/Events?sess=$sess
    let request_url = format!(
        "{base_url}Spokes/DeviceServices/Events?sess={sess}&queue=0",
        base_url = config::CONFIG
            .value_of("url")
            .unwrap_or(config::DEFAULT_URL),
        sess = session_id,
    );
    let mut eventresult: Vec<encoding::DataServiceEvent> = Vec::new();
    let mut response = reqwest::get(&request_url)?;
    let out: encoding::PlantronicsResponse = response.json()?;
    // println!("{:?}", out);
    if !out.is_error && out.type_name == "DeviceEventArray" {
        match out.result {
            Value::Array(outvec) => {
                for item_result in outvec.into_iter() {
                    // println!("item_result is {:?}", item_result);
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
    let _ = config::CONFIG; // Force immediate resolution, in case we need to print help...
    let token = match get_session_id(&"rust-plantronics".to_string()) {
        Ok(token) => token,
        Err(e) => {
            println!("Unable to retrieve token due to err: {}", e);
            return;
        }
    };
    // println!("{:?}", token);
    // Poll loop, forever...
    let delay_time = Duration::from_secs(1);
    let mut worn = true;
    loop {
        if let Ok(events) = get_events(&token) {
            for event in events.into_iter() {
                // println!("ev: {:?}", &event);
                if event.event_name == "MuteOff" && worn {
                    println!("Turning on the sign");
                    reqwest::get(&format!(
                        "{tasmota}cm?cmnd=Power%20On",
                        tasmota = config::CONFIG
                            .value_of("tasmota")
                            .unwrap_or(config::DEFAULT_TAS)
                    ));
                } else if event.event_name == "MuteOn" && worn {
                    println!("Turning off the sign");
                    reqwest::get(&format!(
                        "{tasmota}cm?cmnd=Power%20Off",
                        tasmota = config::CONFIG
                            .value_of("tasmota")
                            .unwrap_or(config::DEFAULT_TAS)
                    ));
                } else if event.event_name == "Doff" {
                    println!("Headset removed");
                    worn = false;
                    reqwest::get(&format!(
                        "{tasmota}cm?cmnd=Power%20Off",
                        tasmota = config::CONFIG
                            .value_of("tasmota")
                            .unwrap_or(config::DEFAULT_TAS)
                    ));
                } else if event.event_name == "Don" {
                    println!("Headset applied to head");
                    worn = true;
                }
            }
        }
        sleep(delay_time);
    }
}
