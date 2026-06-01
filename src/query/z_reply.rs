use super::reply::Reply;
use crate::{ZEncoding, ZReply, ZSample, ZZBytes};
#[cfg(feature = "unstable")]
use crate::ZZenohId;
use prebindgen_proc_macro::prebindgen;

/// Zenoh id of the node that answered, or `None` when unknown (owned handle).
///
/// Unstable: `Reply::replier_id` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn z_reply_replier_zid(r: &ZReply) -> Option<ZZenohId> {
    r.replier_id().map(|id| id.zid())
}

/// Entity id of the replier (0 when the replier is unknown).
///
/// Unstable: `Reply::replier_id` is an `#[unstable]` zenoh API.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn z_reply_replier_eid(r: &ZReply) -> i32 {
    r.replier_id().map(|id| id.eid() as i32).unwrap_or(0)
}

/// `true` if this reply is a success (carries a sample), `false` if an error.
#[prebindgen]
pub fn z_reply_is_ok(r: &ZReply) -> bool {
    r.result().is_ok()
}

/// The reply's sample on success (borrowed; valid while `r` lives), `None` on error.
#[prebindgen]
pub fn z_reply_sample(r: &ZReply) -> Option<&ZSample> {
    r.result().ok()
}

/// The error payload on failure (borrowed bytes), `None` on success.
#[prebindgen]
pub fn z_reply_error_payload(r: &ZReply) -> Option<&ZZBytes> {
    r.result().err().map(|e| e.payload())
}

/// The error encoding on failure (borrowed), `None` on success.
#[prebindgen]
pub fn z_reply_error_encoding(r: &ZReply) -> Option<&ZEncoding> {
    r.result().err().map(|e| e.encoding())
}

/// Decode a native [`ZReply`] into the thick [`Reply`] data class in one hop.
#[prebindgen]
pub fn z_reply_expand(r: &ZReply) -> Reply {
    Reply::from(r)
}
