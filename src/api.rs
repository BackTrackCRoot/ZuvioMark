use failure::Error;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Api {
    client: Client,
    user_info: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RollCall {
    user_id: String,
    #[serde(rename = "accessToken")]
    access_token: String,
    course_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarkRollCall {
    user_id: String,
    #[serde(rename = "accessToken")]
    access_token: String,
    rollcall_id: i32,
}

impl Api {
    pub fn new(user_id: String, access_token: String) -> Self {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("user_id".to_string(), user_id);
        map.insert("accessToken".to_string(), access_token);
        Api {
            client: Client::builder().timeout(None).build().unwrap(),
            user_info: map,
        }
    }
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }
    pub fn get_courses(&self) -> Result<HashMap<String, String>, Error> {
        let res = self
            .client
            .post("http://cty.zuvio.com.cn/index.php/app_v2/getCourseList")
            .headers(self.headers())
            .json(&self.user_info)
            .send()?
            .text()?;
        let ret_json: Value = serde_json::from_str(&res)?;
        let courses = match ret_json["semesters"][0]["courses"].as_array() {
            Some(c) => c,
            None => return Err(format_err!("{}", ret_json["msg"])),
        };
        let mut ret_map: HashMap<String, String> = HashMap::new();
        for i in courses {
            let courses: Value = serde_json::from_str(&i.to_string())?;
            ret_map.insert(
                courses["course_id"].to_string().replace("\"", ""),
                courses["name"].to_string().replace("\"", ""),
            );
        }
        Ok(ret_map)
    }
    pub fn get_rollcall(&self, course_id: String) -> Result<Option<String>, Error> {
        let post_data = RollCall {
            user_id: self.user_info.get("user_id").unwrap().to_string(),
            access_token: self.user_info.get("accessToken").unwrap().to_string(),
            course_id: course_id.parse().unwrap(),
        };

        let res = self
            .client
            .post("http://cty.zuvio.com.cn/index.php/app_v2/getRollcall")
            .headers(self.headers())
            .json(&post_data)
            .send()?
            .text()?;
        let ret_json: Value = serde_json::from_str(&res)?;
        match ret_json.get("rollcall") {
            Some(rc) => {
                if *rc != Value::Null && rc["record"]["answered"] == false {
                    return Ok(Some(rc["id"].to_string().replace("\"", "")));
                }
            }
            None => return Err(format_err!("{}", "Not found rollcall!")),
        }
        Ok(None)
    }
    pub fn mark_rollcall(&self, rollcall_id: String) -> Result<bool, Error> {
        let post_data = MarkRollCall {
            user_id: self.user_info.get("user_id").unwrap().to_string(),
            access_token: self.user_info.get("accessToken").unwrap().to_string(),
            rollcall_id: rollcall_id.parse().unwrap(),
        };

        let res = self
            .client
            .post("http://cty.zuvio.com.cn/index.php/app_v2/makeRollcall")
            .headers(self.headers())
            .json(&post_data)
            .send()?
            .text()?;
        let ret_json: Value = serde_json::from_str(&res)?;
        println!("{:?}", ret_json);
        match ret_json["status"].as_bool() {
            Some(status) => Ok(status),
            None => Ok(false),
        }
    }
}
