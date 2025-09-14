use crate::id::StatusId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    pub id: StatusId,
    pub name: String,
}
