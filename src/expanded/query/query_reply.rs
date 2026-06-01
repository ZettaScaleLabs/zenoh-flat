use crate::{
    Encoding, Error, KeyExpr, ZBytes, ZEncoding, ZQuery, ZZBytes, into_native,
    z_query_reply_delete, z_query_reply_error, z_query_reply_success,
};
use prebindgen_proc_macro::prebindgen;

/// Advanced (ergonomic) twin of [`z_query_reply_success`]: accepts
/// `impl Into<…>` arguments and delegates to the explicit `z_` function.
/// Not wrapped by the C adapter; targets the JNI adapter.
#[prebindgen]
pub fn query_reply_success(
    query: ZQuery,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    payload: impl Into<ZBytes> + Send + 'static,
    encoding: Option<impl Into<Encoding> + Send + 'static>,
    timestamp_ntp64: Option<i64>,
    attachment: Option<impl Into<ZBytes> + Send + 'static>,
    express: Option<bool>,
) -> Result<(), Error> {
    let ke = into_native(key_expr.into())?;
    let payload: ZZBytes = payload.into().into();
    let z_encoding: Option<ZEncoding> = encoding
        .map(|e| e.into().try_into())
        .transpose()?;
    let attachment = attachment.map(|a| ZZBytes::from(a.into()));
    z_query_reply_success(
        &query,
        &ke,
        payload,
        z_encoding.as_ref(),
        timestamp_ntp64,
        attachment,
        express,
    )
}

/// Advanced (ergonomic) twin of [`z_query_reply_error`]. See [`query_reply_success`].
#[prebindgen]
pub fn query_reply_error(
    query: ZQuery,
    payload: impl Into<ZBytes> + Send + 'static,
    encoding: Option<impl Into<Encoding> + Send + 'static>,
) -> Result<(), Error> {
    let payload: ZZBytes = payload.into().into();
    let z_encoding: Option<ZEncoding> = encoding
        .map(|e| e.into().try_into())
        .transpose()?;
    z_query_reply_error(&query, payload, z_encoding.as_ref())
}

/// Advanced (ergonomic) twin of [`z_query_reply_delete`]. See [`query_reply_success`].
#[prebindgen]
pub fn query_reply_delete(
    query: ZQuery,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    timestamp_ntp64: Option<i64>,
    attachment: Option<impl Into<ZBytes> + Send + 'static>,
    express: Option<bool>,
) -> Result<(), Error> {
    let ke = into_native(key_expr.into())?;
    let attachment = attachment.map(|a| ZZBytes::from(a.into()));
    z_query_reply_delete(&query, &ke, timestamp_ntp64, attachment, express)
}
