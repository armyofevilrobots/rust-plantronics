use serde_json::{Error, Value};
use std::thread::sleep;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PlantronicsError {
    pub Description: String,
    #[serde(rename = "Error_Code")]
    pub ErrorCode: u32,
    pub Type: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppRegistration {
    pub Description: String,
    #[serde(default)]
    pub Result: bool,
    #[serde(default)]
    pub error: PlantronicsError,
    pub Type: u32,
    #[serde(rename = "Type_Name")]
    pub typeName: String,
    pub isError: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataServiceEvent {
    /*
        {
        "Age": 1643,
        "Event_Id": 6,
        "Event_Log_Type_Id": 2,
        "Event_Log_Type_Name": "HeadsetStateChange",
        "Event_Name": "MuteOff",
        "Order": 6
    },*/
    pub Age: i32,
    pub Event_Id: i32,
    pub Event_Log_Type_Id: i32,
    pub Event_Log_Type_Name: String,
    pub Event_Name: String,
    pub Order: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantronicsResponse {
    pub Description: String,
    #[serde(default)]
    pub Result: serde_json::Value,
    #[serde(default)]
    pub error: PlantronicsError,
    pub Type: u32,
    #[serde(rename = "Type_Name")]
    pub typeName: String,
    pub isError: bool,
}
