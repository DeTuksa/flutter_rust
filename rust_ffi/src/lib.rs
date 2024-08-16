use serde::{Deserialize, Serialize};
use std::ffi::{CString, CStr};

#[derive(Deserialize, Debug, Serialize)]
pub struct Weather {
    pub temperature: f64,
    pub description: String,
}

#[no_mangle]
pub extern "C" fn fetch_weather(api_url: *const i8) -> *mut i8 {
    let api_url = unsafe { CStr::from_ptr(api_url).to_str().unwrap() };
    let weather: Weather = reqwest::blocking::get(api_url)
        .unwrap()
        .json()
        .unwrap();
    let weather_json = serde_json::to_string(&weather).unwrap();
    CString::new(weather_json).unwrap().into_raw()
}
