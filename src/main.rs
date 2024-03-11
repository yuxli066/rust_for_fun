use rocket::http::Status;
use rocket::serde::json::{json, Value, from_str};
use std::io::{Error,ErrorKind};

// response::Redirect, http::uri::Origin
// encode_path_to_string, decode_to_string 
// const RELEASES_PREFIX: Origin<'static> = uri!("/api/releases");
//  #[get("/")]
//  fn index() -> Redirect {
//     let msg: Option<&str> = None;
//     Redirect::to(uri!(RELEASES_PREFIX, leos_api("windows-x86_64", "v1.0.14", msg)))
//  }

/** 
 * Rust Backend Simple API
 */

 #[macro_use]
 extern crate rocket; 

const API_KEY: &str = "de44da9f17404d9ba7e05611241103";

 #[get("/<_platform>/<_version>?<_msg>")]
 fn leos_api(_platform: &str, _version: &str, _msg: Option<&str>) -> Result<Value, Status> {
    
    // error prone logic -> Option / Result
    if let Some(_msg) = _msg {
        println!("The msg is: \"{:?}\"", _msg);
        return Err(Status::NoContent);
    }

    Ok(json!({
        "notes":"IT WORKS"
    }))
 }

 // weather api wrapper
 #[get("/<_location>")] 
 async fn weather_api(_location: &str) -> Result<Value, std::io::Error> {
    let http_client: reqwest::Client = reqwest::Client::new();
    let aqi: String = String::from("no");
    let uri: String = format!("http://api.weatherapi.com/v1/current.json?key={API_KEY}&q={_location}&aqi={aqi}");
    let _weather_response = http_client
        .get(uri)
        .send()
        .await
        .map_err(|_err|Error::new(ErrorKind::Other,"could not do post req"))?;
    
    if _weather_response.status() == 200 {
        let res_string: String = _weather_response.text().await.map_err(|_err|Error::new(ErrorKind::Other,"could not do post req"))?;
        let res_json: Value = from_str(&res_string)?;
        println!("The weather response body is: \"{:?}\"", res_json);
        Ok(res_json)
    } else {
        return Err(Error::new(ErrorKind::Other, "Unable to retrieve weather api data"));
    }
 }

 #[launch]
 fn rocket() -> _ {
    rocket::build()
        .mount("/api/releases", routes![leos_api])
        .mount("/api/weather", routes![weather_api])
 }
 
