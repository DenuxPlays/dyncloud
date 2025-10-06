use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) struct Providers {
    #[serde(default)]
    pub(crate) cloudflare: Option<Cloudflare>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) struct Cloudflare {
    pub(crate) auth_token: String,
    pub(crate) zone_id: String,
}
