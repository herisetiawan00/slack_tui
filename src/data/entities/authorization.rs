use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuthorizationEntity {
    pub ok: bool,
    pub app_id: String,
    pub authed_user: AuthedUserEntity,
    pub team: TeamEntity,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuthedUserEntity {
    pub id: String,
    pub scope: String,
    pub access_token: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TeamEntity {
    pub id: String,
    pub name: String,
}
