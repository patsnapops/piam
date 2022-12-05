use serde::{Deserialize, Serialize};

use crate::type_alias::IamEntityIdType;

pub type GroupId = IamEntityIdType;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Group {
    pub id: GroupId,
    pub name: String,
}
