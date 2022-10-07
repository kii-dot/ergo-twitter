use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TweetId {
    pub id: String
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageContent {
    pub message: String,
    pub tweet_id: String
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDetails {
    pub nft_id: String,
    pub address: String
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub address: String
}