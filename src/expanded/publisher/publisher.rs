use crate::{
    Encoding, Error, ZBytes, ZEncoding, ZPublisher, ZZBytes, z_publisher_delete, z_publisher_put,
};
use prebindgen_proc_macro::prebindgen;

/// Advanced (ergonomic) twin of [`z_publisher_put`]: accepts `impl Into<…>`
/// payload/encoding/attachment and delegates to the explicit `z_` function.
/// Not wrapped by the C adapter; targets the JNI adapter.
#[prebindgen]
pub fn publisher_put(
    publisher: &ZPublisher,
    payload: impl Into<ZBytes> + Send + 'static,
    encoding: Option<impl Into<Encoding> + Send + 'static>,
    attachment: Option<impl Into<ZBytes> + Send + 'static>,
) -> Result<(), Error> {
    let payload: ZZBytes = payload.into().into();
    let z_encoding: Option<ZEncoding> = encoding
        .map(|e| e.into().try_into())
        .transpose()?;
    let attachment = attachment.map(|a| ZZBytes::from(a.into()));
    z_publisher_put(publisher, payload, z_encoding.as_ref(), attachment)
}

/// Advanced (ergonomic) twin of [`z_publisher_delete`]. See [`publisher_put`].
#[prebindgen]
pub fn publisher_delete(
    publisher: &ZPublisher,
    attachment: Option<impl Into<ZBytes> + Send + 'static>,
) -> Result<(), Error> {
    let attachment = attachment.map(|a| ZZBytes::from(a.into()));
    z_publisher_delete(publisher, attachment)
}
