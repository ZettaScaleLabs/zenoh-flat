use crate::{Encoding, Error, ZBytes, ZEncoding, ZPublisher, ZZBytes};
use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

#[prebindgen]
pub fn z_publisher_put(
    publisher: &ZPublisher,
    payload: ZZBytes,
    encoding: &ZEncoding,
    attachment: Option<ZZBytes>,
) -> Result<(), Error> {
    let mut publication = publisher.put(payload).encoding(encoding.clone());
    if let Some(att) = attachment {
        publication = publication.attachment(att);
    }
    publication.wait().map_err(Error::from)
}

#[prebindgen]
pub fn z_publisher_delete(
    publisher: &ZPublisher,
    attachment: Option<ZZBytes>,
) -> Result<(), Error> {
    let mut delete = publisher.delete();
    if let Some(att) = attachment {
        delete = delete.attachment(att);
    }
    delete.wait().map_err(Error::from)
}

/// Advanced (ergonomic) twin of [`z_publisher_put`]: accepts `impl Into<…>`
/// payload/encoding/attachment and delegates to the explicit `z_` function.
/// Not wrapped by the C adapter; targets the JNI adapter.
#[prebindgen]
pub fn publisher_put(
    publisher: &ZPublisher,
    payload: impl Into<ZBytes> + Send + 'static,
    encoding: impl Into<Encoding> + Send + 'static,
    attachment: Option<impl Into<ZBytes> + Send + 'static>,
) -> Result<(), Error> {
    let payload: ZZBytes = payload.into().into();
    let z_encoding: ZEncoding = encoding.into().try_into()?;
    let attachment = attachment.map(|a| ZZBytes::from(a.into()));
    z_publisher_put(publisher, payload, &z_encoding, attachment)
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
