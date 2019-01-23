extern crate clap;

use clap::{App, Arg, ArgMatches};

pub static DEFAULT_URL: &str = &"http://localhost:32017/";
pub static DEFAULT_TAS: &str = &"http://sonoff-on-air.local/";
pub static APP_UID: &str = &"ecfc7bc4-d9d2-431c-ab6d-173bd6f3fd61";

lazy_static! {
    pub static ref CONFIG: ArgMatches<'static> = App::new("rust-plantronics")
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
        .arg(
            Arg::with_name("tasmota")
                .short("T")
                .long("tasmota")
                .takes_value(true)
                .required(true)
                .help("The destination url for the tasmota rest api (http://sonoff-on-air.local/)"),
        )
        .get_matches();
}
