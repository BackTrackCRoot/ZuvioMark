use failure::Error;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use serde_json::Value;

#[derive(Debug)]
pub struct Api {
    client: Client,
    user_info: UserInfo,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct UserInfo {
    pub user_id: String,
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RollCall {
    #[serde(flatten)]
    user_info: UserInfo,
    course_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarkRollCall {
    #[serde(flatten)]
    user_info: UserInfo,
    rollcall_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    pub course_id: u32,
    pub name: String,
}

impl Api {
    pub fn new(user_info: UserInfo) -> Self {
        let client = Client::builder().timeout(None).build().unwrap();

        Api { client, user_info }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }

    pub fn get_courses(&self) -> Result<Vec<Course>, Error> {
        let res = self
            .client
            .post("http://cty.zuvio.com.cn/index.php/app_v2/getCourseList")
            .headers(self.headers())
            .json(&self.user_info)
            .send()?
            .text()?;

        let ret_json: Value = serde_json::from_str(&res)?;
        let courses_json = &ret_json["semesters"][0]["courses"];

        if courses_json.is_null() {
            return Err(format_err!("{}", ret_json["msg"]));
        }

        let courese: Vec<Course> = serde_json::from_value(courses_json.clone())?;

        Ok(courese)
    }

    pub fn get_rollcall(&self, course_id: u32) -> Result<Option<u32>, Error> {
        let post_data = RollCall {
            user_info: self.user_info.clone(),
            course_id,
        };

        let res = self
            .client
            .post("http://cty.zuvio.com.cn/index.php/app_v2/getRollcall")
            .headers(self.headers())
            .json(&post_data)
            .send()?
            .text()?;

        let ret_json: Value = serde_json::from_str(&res)?;
        let rollcall = ret_json
            .get("rollcall")
            .ok_or_else(|| format_err!("Not found rollcall!"))?;
        let answered = rollcall["record"]["answered"]
            .as_bool()
            .ok_or_else(|| format_err!("Not found answered!"))?;

        if !answered {
            Ok(Some(rollcall["id"].as_u64().unwrap() as u32))
        } else {
            Ok(None)
        }
    }

    pub fn mark_rollcall(&self, rollcall_id: u32) -> Result<bool, Error> {
        let post_data = MarkRollCall {
            user_info: self.user_info.clone(),
            rollcall_id,
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

        Ok(ret_json["status"].as_bool().unwrap_or(false))
    }
}
