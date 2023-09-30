use crate::model::JwtToken;
use entity::res::ResJwtToken;

impl From<JwtToken> for ResJwtToken {
    fn from(model: JwtToken) -> Self {
        Self {
            token: model.token,
        }
    }
}

impl From<&JwtToken> for ResJwtToken {
    fn from(model: &JwtToken) -> Self {
        Self {
            token: model.token.clone(),
        }
    }
}
