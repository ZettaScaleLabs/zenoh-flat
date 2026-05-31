use crate::{into_native, Encoding, Error, KeyExpr, ZBytes, ZEncoding, ZKeyExpr, ZQuery, ZZBytes};
use prebindgen_proc_macro::prebindgen;
use zenoh::{
    time::{Timestamp, TimestampId, NTP64},
    Wait,
};

#[prebindgen]
pub fn z_query_reply_success(
    query: ZQuery,
    key_expr: &ZKeyExpr,
    payload: ZZBytes,
    encoding: &ZEncoding,
    timestamp_ntp64: Option<i64>,
    attachment: Option<ZZBytes>,
    express: bool,
) -> Result<(), Error> {
    let mut b = query.reply(key_expr, payload).encoding(encoding.clone());
    if let Some(ntp) = timestamp_ntp64 {
        b = b.timestamp(Timestamp::new(NTP64(ntp as u64), TimestampId::rand()));
    }
    if let Some(att) = attachment {
        b = b.attachment(att);
    }
    b.express(express).wait().map_err(Error::from)
}

#[prebindgen]
pub fn z_query_reply_error(
    query: ZQuery,
    payload: ZZBytes,
    encoding: &ZEncoding,
) -> Result<(), Error> {
    query
        .reply_err(payload)
        .encoding(encoding.clone())
        .wait()
        .map_err(Error::from)
}

#[prebindgen]
pub fn z_query_reply_delete(
    query: ZQuery,
    key_expr: &ZKeyExpr,
    timestamp_ntp64: Option<i64>,
    attachment: Option<ZZBytes>,
    express: bool,
) -> Result<(), Error> {
    let mut b = query.reply_del(key_expr);
    if let Some(ntp) = timestamp_ntp64 {
        b = b.timestamp(Timestamp::new(NTP64(ntp as u64), TimestampId::rand()));
    }
    if let Some(att) = attachment {
        b = b.attachment(att);
    }
    b.express(express).wait().map_err(Error::from)
}

/// Advanced (ergonomic) twin of [`z_query_reply_success`]: accepts
/// `impl Into<…>` arguments and delegates to the explicit `z_` function.
/// Not wrapped by the C adapter; targets the JNI adapter.
#[prebindgen]
pub fn query_reply_success(
    query: ZQuery,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    payload: impl Into<ZBytes> + Send + 'static,
    encoding: impl Into<Encoding> + Send + 'static,
    timestamp_ntp64: Option<i64>,
    attachment: Option<impl Into<ZBytes> + Send + 'static>,
    express: bool,
) -> Result<(), Error> {
    let ke = into_native(key_expr.into())?;
    let payload: ZZBytes = payload.into().into();
    let z_encoding: ZEncoding = encoding.into().try_into()?;
    let attachment = attachment.map(|a| ZZBytes::from(a.into()));
    z_query_reply_success(query, &ke, payload, &z_encoding, timestamp_ntp64, attachment, express)
}

/// Advanced (ergonomic) twin of [`z_query_reply_error`]. See [`query_reply_success`].
#[prebindgen]
pub fn query_reply_error(
    query: ZQuery,
    payload: impl Into<ZBytes> + Send + 'static,
    encoding: impl Into<Encoding> + Send + 'static,
) -> Result<(), Error> {
    let payload: ZZBytes = payload.into().into();
    let z_encoding: ZEncoding = encoding.into().try_into()?;
    z_query_reply_error(query, payload, &z_encoding)
}

/// Advanced (ergonomic) twin of [`z_query_reply_delete`]. See [`query_reply_success`].
#[prebindgen]
pub fn query_reply_delete(
    query: ZQuery,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    timestamp_ntp64: Option<i64>,
    attachment: Option<impl Into<ZBytes> + Send + 'static>,
    express: bool,
) -> Result<(), Error> {
    let ke = into_native(key_expr.into())?;
    let attachment = attachment.map(|a| ZZBytes::from(a.into()));
    z_query_reply_delete(query, &ke, timestamp_ntp64, attachment, express)
}
