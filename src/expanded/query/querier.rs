use crate::{Encoding, Error, Reply, ZBytes, ZEncoding, ZQuerier, ZZBytes, z_querier_get};
use prebindgen_proc_macro::prebindgen;

/// Perform a GET through a querier, delivering each reply as a fully decoded
/// [`Reply`] data class (thick surface — one FFI hop per reply for
/// expensive-FFI bindings). See [`z_querier_get`] for parameter semantics.
#[prebindgen]
pub fn querier_get(
    querier: &ZQuerier,
    parameters: Option<String>,
    payload: Option<impl Into<ZBytes> + Send + 'static>,
    encoding: impl Into<Encoding> + Send + 'static,
    attachment: Option<impl Into<ZBytes> + Send + 'static>,
    callback: impl Fn(Reply) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), Error> {
    let payload = payload.map(|p| ZZBytes::from(p.into()));
    let z_encoding: ZEncoding = encoding.into().try_into()?;
    let attachment = attachment.map(|a| ZZBytes::from(a.into()));
    z_querier_get(
        querier,
        parameters,
        payload,
        &z_encoding,
        attachment,
        move |zr| callback(Reply::from(&zr)),
        on_close,
    )
}
