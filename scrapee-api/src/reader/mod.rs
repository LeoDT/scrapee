use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum ReaderBlockConfig {
    Site { site_id: i32 },
}
