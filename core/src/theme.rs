use serde::Deserialize;

#[derive(Deserialize)]
pub struct Theme {

    pub background: String,
    pub text: String,
    pub keyword: String,
    pub string: String,
}