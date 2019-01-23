#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PlantronicsError {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Error_Code")]
    pub error_code: u32,
    #[serde(rename = "Type")]
    pub error_type: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppRegistration {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(default)]
    #[serde(rename = "Result")]
    pub result: bool,
    #[serde(default)]
    pub error: PlantronicsError,
    #[serde(rename = "Type")]
    pub result_type: u32,
    #[serde(rename = "Type_Name")]
    pub type_name: String,
    #[serde(rename = "isError")]
    pub is_error: bool,
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
    #[serde(rename = "Age")]
    pub age: i32,
    #[serde(rename = "Event_Id")]
    pub event_id: i32,
    #[serde(rename = "Event_Log_Type_Id")]
    pub event_log_type_id: i32,
    #[serde(rename = "Event_Log_Type_Name")]
    pub event_log_type_name: String,
    #[serde(rename = "Event_Name")]
    pub event_name: String,
    #[serde(rename = "Order")]
    pub order: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantronicsResponse {
    #[serde(default)]
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(default)]
    #[serde(rename = "Result")]
    pub result: serde_json::Value,
    #[serde(default)]
    pub error: PlantronicsError,
    #[serde(rename = "Type")]
    pub result_type: u32,
    #[serde(rename = "Type_Name")]
    pub type_name: String,
    #[serde(rename = "isError")]
    pub is_error: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantronicsState {}
