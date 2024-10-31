use reqwest;

pub struct ResBody {
    pub success: bool,
    pub code: reqwest::StatusCode,
    pub msg: String,
}