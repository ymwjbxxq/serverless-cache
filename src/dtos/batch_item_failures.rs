use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct BatchItemFailures {
    #[serde(rename = "batchItemFailures")]
    pub batch_item_failures: Vec<ItemIdentifier>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ItemIdentifier {
    #[serde(rename = "itemIdentifier")]
    pub item_identifier: String,
}