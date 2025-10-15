use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) struct Providers {
    #[serde(default)]
    #[validate(nested)]
    pub(crate) cloudflare: Option<Cloudflare>,
}

#[derive(Debug, Deserialize, Validate)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) struct Cloudflare {
    #[validate(length(min = 1))]
    pub(crate) auth_token: String,
    #[validate(length(min = 1))]
    pub(crate) zone_id: String,
}
