#[macro_use]
extern crate serde_derive;
extern crate reqwest;
use std::thread::sleep;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Serialize, Deserialize, Debug, Default)]
struct PlantronicsError {
    Description: String,
    #[serde(rename = "Error_Code")]
    ErrorCode: u32,
    Type: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct AppRegistration {
    Description: String,
    #[serde(default)]
    Result: bool,
    #[serde(default)]
    error: PlantronicsError,
    Type: u32,
    #[serde(rename = "Type_Name")]
    typeName: String,
    isError: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct SessionResponse {
    Description: String,
    #[serde(default)]
    Result: String,
    #[serde(default)]
    error: PlantronicsError,
    Type: u32,
    #[serde(rename = "Type_Name")]
    typeName: String,
    isError: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataServiceEvent {
    /*
        {
            "Age": 1643,
            "Event_Id": 6,
            "Event_Log_Type_Id": 2,
            "Event_Log_Type_Name": "HeadsetStateChange",
            "Event_Name": "MuteOff",
            "Order": 6
    },*/
    Age: i32,
    Event_Id: i32,
    Event_Log_Type_Id: i32,
    Event_Log_Type_Name: String,
    Event_Name: String,
    Order: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataServiceResponse {
    Description: String,
    #[serde(default)]
    Result: Vec<DataServiceEvent>,
    #[serde(default)]
    error: PlantronicsError,
    Type: u32,
    #[serde(rename = "Type_Name")]
    typeName: String,
    isError: bool,
}

fn get_session_id(name: &String) -> Result<String> {
    let request_url = format!(
        "http://localhost:32017/Spokes/SessionManager/Register?name={name}",
        name = name
    );
    println!("{}", request_url);
    let mut response = reqwest::get(&request_url)?;
    let out: AppRegistration = response.json()?;
    println!("{:?}", out);
    let request_url = format!("http://localhost:32017/Spokes/DeviceServices/Attach?uid=0123456789");
    let mut response = reqwest::get(&request_url)?;
    let out: SessionResponse = response.json()?;
    println!("{:?}", out);
    if !out.isError && out.typeName == "SessionHash" {
        let session_id = out.Result;
        if session_id == "" {
            return Err("No session ID in the result.".into());
        } else {
            return Ok(session_id);
        }
    } else {
        return Err("Undefined error from session endpoint.".into());
    }
}

fn get_events(session_id: &String) -> Result<Vec<DataServiceEvent>> {
    // Wherein we http://localhost:32017/Spokes/DeviceServices/Events\?sess\=6ba5b8b0bb6d45bd574cd1fc4f1bb024
    // This will return:
    /*
        {
        "Description": "Device Events",
        "Result": [
        {
        "Age": 1643,
        "Event_Id": 6,
        "Event_Log_Type_Id": 2,
        "Event_Log_Type_Name": "HeadsetStateChange",
        "Event_Name": "MuteOff",
        "Order": 6
        },
        {
        "Age": 1647,
        "Event_Id": 5,
        "Event_Log_Type_Id": 4,
        "Event_Log_Type_Name": "HeadsetButtonPressed",
        "Event_Name": "Mute",
        "Order": 5
        }
    ],
        "Type": 6,
        "Type_Name": "DeviceEventArray",
        "isError": false
    }

        // OR

        {
        "Description": "Device Events",
        "Result": [
        {
        "Age": 1199,
        "Event_Id": 5,
        "Event_Log_Type_Id": 2,
        "Event_Log_Type_Name": "HeadsetStateChange",
        "Event_Name": "MuteOn",
        "Order": 4
    },
        {
        "Age": 1202,
        "Event_Id": 5,
        "Event_Log_Type_Id": 4,
        "Event_Log_Type_Name": "HeadsetButtonPressed",
        "Event_Name": "Mute",
        "Order": 3
    }
    ],
        "Type": 6,
        "Type_Name": "DeviceEventArray",
        "isError": false
    }
         */
    let request_url = format!(
        "http://localhost:32017/Spokes/DeviceServices/Events?sess={sess}",
        sess = session_id
    );
    let mut response = reqwest::get(&request_url)?;
    let out: DataServiceResponse = response.json()?;
    println!("{:?}", out);
    if !out.isError && out.typeName == "DeviceEventArray" {
        let eventresult = out.Result;
        return Ok(eventresult);
    } else {
        return Err("Undefined error from session endpoint.".into());
    }
}

fn main() {
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
            println!("{:?}", event);
        }
        sleep(delay_time);
    }
}
