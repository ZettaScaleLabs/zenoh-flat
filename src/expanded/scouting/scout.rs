use crate::{Error, Hello, ZConfig, ZScout, z_scout};
use prebindgen_proc_macro::prebindgen;

/// Start a scout, delivering each hello as a serialized [`Hello`] data
/// class. See [`z_scout`] for parameter semantics; this variant pays one
/// extra Rust-side `Hello::from(ZHello)` copy per message in exchange for
/// a single FFI hop on the binding side.
#[prebindgen]
pub fn scout(
    whatami: i32,
    config: Option<&ZConfig>,
    callback: impl Fn(Hello) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<ZScout, Error> {
    z_scout(
        whatami,
        config,
        move |zh| callback(Hello::from(zh)),
        on_close,
    )
}
