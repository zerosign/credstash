use dynomite::{
    DynamoDbExt, FromAttributes, Item
}

#[derive(Debug, Item, Clone)]
pub struct Secret {
    contents: Vec<u8>,
    hmac: Vec<u8>,
    version: String,
}
