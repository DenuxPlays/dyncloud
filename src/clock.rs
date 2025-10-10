use chrono::{FixedOffset, Local, Offset};
use std::sync::LazyLock;

static SYSTEM_OFFSET: LazyLock<FixedOffset> = LazyLock::new(|| Local::now().offset().fix());

pub(crate) fn get_system_timezone_offset() -> FixedOffset {
    *SYSTEM_OFFSET
}
