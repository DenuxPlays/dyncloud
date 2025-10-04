use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) enum Resolver {
    Ipfiy,
}

impl Default for Resolver {
    fn default() -> Self {
        Self::Ipfiy
    }
}
