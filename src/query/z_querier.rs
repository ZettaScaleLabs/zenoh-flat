use super::reply::Reply;
use crate::util::OnceDrop;
use crate::{Encoding, Error, ZBytes, ZEncoding, ZQuerier, ZReply, ZZBytes};
use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

/// Perform a GET through a querier, delivering each reply as an opaque
/// [`ZReply`] handle (thin surface — cheap-FFI bindings pull fields via the
/// `z_reply_*` accessors). `on_close` fires when the reply stream ends.
#[prebindgen]
pub fn z_querier_get(
    querier: &ZQuerier,
    parameters: Option<String>,
    payload: Option<ZZBytes>,
    encoding: &ZEncoding,
    attachment: Option<ZZBytes>,
    callback: impl Fn(ZReply) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), Error> {
    let on_close = OnceDrop::new(on_close);
    let mut builder = querier.get();
    if let Some(params) = parameters {
        builder = builder.parameters(params);
    }
    if let Some(payload) = payload {
        builder = builder.payload(payload).encoding(encoding.clone());
    }
    if let Some(attachment) = attachment {
        builder = builder.attachment(attachment);
    }
    builder
        .callback(move |reply| {
            let _ = &on_close;
            callback(reply);
        })
        .wait()
        .map_err(Error::from)
}

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
