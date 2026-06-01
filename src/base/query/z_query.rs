use crate::{Error, ZEncoding, ZKeyExpr, ZQuery, ZZBytes};
use prebindgen_proc_macro::prebindgen;
use zenoh::{
    Wait,
    time::{NTP64, Timestamp, TimestampId},
};

/// Key expression the query targets (borrowed; valid while `q` lives).
#[prebindgen]
pub fn z_query_keyexpr(q: &ZQuery) -> &ZKeyExpr {
    q.key_expr()
}

/// Query selector parameters as an owned string (empty when none).
#[prebindgen]
pub fn z_query_parameters(q: &ZQuery) -> String {
    q.parameters().as_str().to_string()
}

/// Query payload (borrowed bytes), or `None` when the query carries none.
#[prebindgen]
pub fn z_query_payload(q: &ZQuery) -> Option<&ZZBytes> {
    q.payload()
}

/// Encoding of the query payload (borrowed), or `None`.
#[prebindgen]
pub fn z_query_encoding(q: &ZQuery) -> Option<&ZEncoding> {
    q.encoding()
}

#[prebindgen]
pub fn z_query_reply_success(
    query: &ZQuery,
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
    query: &ZQuery,
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
    query: &ZQuery,
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
