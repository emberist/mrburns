use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Author {
    pub id: i32,
}

#[derive(Deserialize, Debug)]
pub struct CreateMrResponse {
    pub iid: i32,
    pub web_url: String,
    pub author: Author,
}

#[derive(Serialize, Debug)]
pub struct CreateMrBody {
    pub title: String,
    pub description: String,
    pub source_branch: String,
    pub target_branch: String,
    pub draft: bool,
    pub remove_source_branch: bool,
    pub squash_on_merge: bool,
}
