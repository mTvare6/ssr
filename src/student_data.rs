use serde::{Serialize, Deserialize};
use serde_json::{Map,Value};
use either::Either;
use directories::ProjectDirs;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize)]
#[allow(non_snake_case)]
struct DBConfig{
    dataSource: String,
    database: String,
    collection: String,
    filter: Map<String, Value>,
    limit: u64,
}
#[derive(Serialize)]
struct ApiKeyBody {
   key: String,
}
#[allow(dead_code)]
#[derive(Deserialize)]
struct AccessTokenBody{
    access_token: String,
    refresh_token: String,
    user_id: String,
    device_id: String,
}
#[derive(Deserialize)]
#[derive(Debug)]
pub struct ResponseBody{
    pub documents: Vec<Student>
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[derive(Debug,Clone)]
pub struct Student{
    _id : String,
    pub a: Option<String>,
    pub b: String,
    pub d: String,
    pub g: String,
    pub h: String,
    pub i: String,
    pub n: String,
    pub p: String,
    pub r: String,
    pub u: String,
    #[serde(with = "either::serde_untagged")]
    c: Either<String, Vec<String>>,
}

fn get_student_response() -> Result<String, Box<dyn std::error::Error>> {
    let search_config = DBConfig{
        dataSource: String::from("Cluster0"),
        database: String::from("student_search"),
        collection: String::from("student_search"),
        filter: Map::new(),
        limit: 30000,
    };
    let api_id = "data-rgzxa";
    let key_body = ApiKeyBody{ key: String::from("BFXDWTSqEFOXbk5kzCYsqV50vL4YjUQywDUfcE5wy0cD01SsJ7nLFduSzjE4Or3W") };
    let recv_body = ureq::post(format!("https://ap-south-1.aws.realm.mongodb.com/api/client/v2.0/app/{}/auth/providers/api-key/login", api_id))
        .header("Content-Type", "application/json")
        .send_json(&key_body)?
        .body_mut()
        .read_json::<AccessTokenBody>()?;
    let response = ureq::post(format!("https://ap-south-1.aws.data.mongodb-api.com/app/{}/endpoint/data/v1/action/find", api_id))
        .header("Authorization", format!("Bearer {}", recv_body.access_token))
        .send_json(&search_config)?
        .body_mut()
        .read_to_string()?;
    Ok(response)
}

pub fn get_student_data_json() -> Result<ResponseBody, Box<dyn std::error::Error>> {
    if let Some(project_dirs) = ProjectDirs::from("me", "mtvare6", "ssr"){
        if std::fs::create_dir_all(project_dirs.data_local_dir()).is_err() {
            Ok(serde_json::from_str(&get_student_response()?)?)
        }
        else{
            let json_file = format!("{}/main.json", project_dirs.data_local_dir().display());
            if let Ok(mut file) = File::open(json_file.clone()){
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                Ok(serde_json::from_str(&contents)?)
            }
            else{
                let mut file = File::create_new(json_file)?;
                let response_data = get_student_response()?;
                let _ = file.write_all(response_data.as_bytes());
                Ok(serde_json::from_str(&response_data)?)
            }
        }
    }
    else{
        Ok(serde_json::from_str(&get_student_response()?)?)
    }
}

