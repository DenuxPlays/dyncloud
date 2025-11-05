use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
pub(crate) enum Resolver {
    #[default]
    Ipfiy,
}
