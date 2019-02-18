#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate pretty_env_logger;
extern crate reqwest;
#[macro_use]
extern crate log;
extern crate dns_lookup;
//extern crate mdns;
extern crate rand;
extern crate url;

// use reqwest;
use rand::{thread_rng, Rng};
use serde_json::Value;
use std::env;
use std::net::IpAddr;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use url::{Host, ParseError, Url};

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
    debug!("{}", request_url);
    let mut response = reqwest::get(&request_url)?;
    let out: encoding::AppRegistration = response.json()?;
    debug!("{:?}", out);
    let request_url = format!(
        "{baseurl}Spokes/DeviceServices/Attach?uid={app_uid}",
        baseurl = config::CONFIG
            .value_of("url")
            .unwrap_or(config::DEFAULT_URL),
        app_uid = config::APP_UID,
    );
    let mut response = reqwest::get(&request_url)?;
    let out: encoding::PlantronicsResponse = response.json()?;
    // info!("{:?}", out);
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
    // info!("{:?}", &json_str);
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
    // info!("{:?}", out);
    if !out.is_error && out.type_name == "DeviceEventArray" {
        match out.result {
            Value::Array(outvec) => {
                for item_result in outvec.into_iter() {
                    // info!("item_result is {:?}", item_result);
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

fn bonjour_lookup_host(hostname: &str) -> Result<String> {
    let mut ips: Vec<std::net::IpAddr> = dns_lookup::lookup_host(hostname)?;
    // Oh crap, this order may be stable, but we want to randomly try them
    // in case one IP is broken/unroutable, or whatevs.
    // let _ips: &[std::net::IpAddr] = ips.as_mut_slice();
    thread_rng().shuffle(&mut ips);
    for ip in ips {
        debug!("An IP: {:?}", ip);
        return Ok(ip.to_string());
    }
    Ok(hostname.into())
}

fn rewrite_url_with_mdns(url: &str) -> Result<String> {
    let mut url_obj = Url::parse(url)?;
    match url_obj.host() {
        Some(Host::Domain(host_name)) => {
            debug!("Rewriting URL host {}", &host_name);
            let host = bonjour_lookup_host(&host_name)?;
            url_obj.set_host(Some(&host));
            info!("Rewrote URL to {}", url_obj.as_str());
            return Ok(url_obj.to_string());
        }
        Some(Host::Ipv4(addr)) => {
            debug!("Received an IPV4 host {}, no rewrite.", &addr);
        }
        Some(Host::Ipv6(addr)) => {
            debug!("Received an IPV6 host {}, fancy!", &addr);
        }
        None => {}
    }
    warn!(
        "Could not lookup host for {}, so using raw, may be flaky!",
        url
    );
    Ok(url.into())
}

fn main() {
    // Set the default env var the easy way...
    match env::var_os("RUST_LOG") {
        Some(_) => {}
        None => {
            env::set_var("RUST_LOG", "info");
        }
    }

    // Let's use mDNS to lookup the host eh?

    // Initialize logging
    pretty_env_logger::init();
    loop {
        let _ = config::CONFIG; // Force immediate resolution, in case we need to print help...
        let token = match get_session_id(&"rust-plantronics".to_string()) {
            Ok(token) => token,
            Err(e) => {
                error!("Unable to retrieve token due to err: {}", e);
                break;
            }
        };
        debug!("Token is {:?}", token);
        info!("TOKEN: {:?}", &token);
        // Poll loop, forever...
        let delay_time = Duration::from_secs(1);
        let mut worn = true;

        let tasmota = match rewrite_url_with_mdns(
            config::CONFIG
                .value_of("tasmota")
                .unwrap_or(config::DEFAULT_TAS),
        ) {
            Ok(tasmota) => tasmota,
            Err(e) => {
                error!("Unable to lookup host :( {}", e);
                break;
            }
        };
        loop {
            if let Ok(events) = get_events(&token) {
                for event in events.into_iter() {
                    // info!("ev: {:?}", &event);
                    if event.event_name == "MuteOff" && worn {
                        info!("Turning on the sign");
                        let result = reqwest::get(&format!(
                            "{tasmota}cm?cmnd=Power%20On",
                            tasmota = tasmota
                        ));
                        match result {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Got an error retrieving result for events: {}", e);
                                break;
                            }
                        }
                    } else if event.event_name == "MuteOn" && worn {
                        info!("Turning off the sign");
                        let result = reqwest::get(&format!(
                            "{tasmota}cm?cmnd=Power%20Off",
                            tasmota = tasmota
                        ));
                        match result {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Got an error turning off the Tasmota! {}", e);
                                break;
                            }
                        }
                    } else if event.event_name == "Doff" {
                        info!("Headset removed");
                        worn = false;
                        let result = reqwest::get(&format!(
                            "{tasmota}cm?cmnd=Power%20Off",
                            tasmota = tasmota
                        ));
                        match result {
                            Ok(_) => {}
                            Err(e) => error!("Got an error turning off the Tasmota! {}", e),
                        }
                    } else if event.event_name == "Don" {
                        info!("Headset applied to head");
                        worn = true;
                    }
                }
            }
            let now = SystemTime::now();
            sleep(delay_time);
            match now.elapsed() {
                Ok(elapsed) => {
                    if elapsed.as_secs() > 30 {
                        error!(
                            "Unexpectedly long delay of {} seconds. Restarting.",
                            elapsed.as_secs()
                        );
                        break;
                    }
                }
                Err(e) => {
                    info!("Error occured {:?}", e);
                    break;
                }
            }
        }
        warn!("We exited the polling loop. Restarting...")
    }
}
