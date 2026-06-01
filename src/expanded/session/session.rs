use crate::{
    CongestionControl, ConsolidationMode, Encoding, Error, KeyExpr, Priority, Query, QueryTarget,
    Reply, ReplyKeyExpr, Sample, ZBytes, ZEncoding, ZQuerier, ZQueryable, ZSession, ZSubscriber,
    ZZBytes, ZenohId, into_native, z_session_declare_querier, z_session_declare_queryable,
    z_session_declare_subscriber, z_session_get,
};
#[cfg(feature = "unstable")]
use crate::{
    Reliability, ZPublisher, z_session_declare_publisher, z_session_delete, z_session_put,
};
use prebindgen_proc_macro::prebindgen;
use zenoh::Wait;

/// Declare a subscriber delivering each change as a fully decoded [`Sample`]
/// data class (thick surface). See [`z_session_declare_subscriber`].
#[prebindgen]
pub fn session_declare_subscriber(
    session: &ZSession,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    callback: impl Fn(Sample) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<ZSubscriber, Error> {
    let ke = into_native(key_expr.into())?;
    z_session_declare_subscriber(session, ke, move |zs| callback(Sample::from(&zs)), on_close)
}

/// Declare a queryable delivering each query as a fully decoded [`Query`] data
/// class (thick surface). See [`z_session_declare_queryable`].
#[prebindgen]
pub fn session_declare_queryable(
    session: &ZSession,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    complete: Option<bool>,
    callback: impl Fn(Query) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<ZQueryable, Error> {
    let ke = into_native(key_expr.into())?;
    z_session_declare_queryable(
        session,
        ke,
        complete,
        move |zq| callback(Query::from(zq)),
        on_close,
    )
}

/// Query matching queryables, delivering each reply as a fully decoded
/// [`Reply`] data class (thick surface). See [`z_session_get`].
#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn session_get(
    session: &ZSession,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    parameters: Option<String>,
    timeout_ms: Option<i64>,
    target: Option<QueryTarget>,
    consolidation: Option<ConsolidationMode>,
    accept_replies: Option<ReplyKeyExpr>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    payload: Option<impl Into<ZBytes> + Send + 'static>,
    encoding: Option<impl Into<Encoding> + Send + 'static>,
    attachment: Option<impl Into<ZBytes> + Send + 'static>,
    callback: impl Fn(Reply) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), Error> {
    let ke = into_native(key_expr.into())?;
    let payload = payload.map(|p| ZZBytes::from(p.into()));
    let z_encoding: Option<ZEncoding> = encoding
        .map(|e| e.into().try_into())
        .transpose()?;
    let attachment = attachment.map(|a| ZZBytes::from(a.into()));
    z_session_get(
        session,
        &ke,
        parameters,
        timeout_ms,
        target,
        consolidation,
        accept_replies,
        congestion_control,
        priority,
        express,
        payload,
        z_encoding.as_ref(),
        attachment,
        move |zr| callback(Reply::from(&zr)),
        on_close,
    )
}

/// Advanced (ergonomic) twin of [`z_session_declare_publisher`]: accepts
/// `impl Into<KeyExpr>` and delegates to the explicit `z_` function. Not
/// wrapped by the C adapter; targets the JNI adapter.
///
/// Unstable: carries the `reliability` QoS param (see [`z_session_declare_publisher`]).
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn session_declare_publisher(
    session: &ZSession,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    reliability: Option<Reliability>,
) -> Result<ZPublisher, Error> {
    let ke = into_native(key_expr.into())?;
    z_session_declare_publisher(
        session,
        ke,
        congestion_control,
        priority,
        express,
        reliability,
    )
}

/// Advanced (ergonomic) twin of [`z_session_put`]. See [`session_declare_publisher`].
///
/// Unstable: carries the `reliability` QoS param.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
#[allow(clippy::too_many_arguments)]
pub fn session_put(
    session: &ZSession,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    payload: impl Into<ZBytes> + Send + 'static,
    encoding: Option<impl Into<Encoding> + Send + 'static>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    attachment: Option<impl Into<ZBytes> + Send + 'static>,
    reliability: Option<Reliability>,
) -> Result<(), Error> {
    let ke = into_native(key_expr.into())?;
    let payload: ZZBytes = payload.into().into();
    let z_encoding: Option<ZEncoding> = encoding
        .map(|e| e.into().try_into())
        .transpose()?;
    let attachment = attachment.map(|a| ZZBytes::from(a.into()));
    z_session_put(
        session,
        &ke,
        payload,
        z_encoding.as_ref(),
        congestion_control,
        priority,
        express,
        attachment,
        reliability,
    )
}

/// Advanced (ergonomic) twin of [`z_session_delete`]. See [`session_declare_publisher`].
///
/// Unstable: carries the `reliability` QoS param.
#[cfg(feature = "unstable")]
#[prebindgen(cfg = "feature = \"unstable\"")]
pub fn session_delete(
    session: &ZSession,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    attachment: Option<impl Into<ZBytes> + Send + 'static>,
    reliability: Option<Reliability>,
) -> Result<(), Error> {
    let ke = into_native(key_expr.into())?;
    let attachment = attachment.map(|a| ZZBytes::from(a.into()));
    z_session_delete(
        session,
        &ke,
        congestion_control,
        priority,
        express,
        attachment,
        reliability,
    )
}

/// Advanced (ergonomic) twin of [`z_session_declare_querier`]. See [`session_declare_publisher`].
#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn session_declare_querier(
    session: &ZSession,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    target: Option<QueryTarget>,
    consolidation: Option<ConsolidationMode>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    timeout_ms: Option<i64>,
    accept_replies: Option<ReplyKeyExpr>,
) -> Result<ZQuerier, Error> {
    let ke = into_native(key_expr.into())?;
    z_session_declare_querier(
        session,
        ke,
        target,
        consolidation,
        congestion_control,
        priority,
        express,
        timeout_ms,
        accept_replies,
    )
}

// Zid accessors come in two tiers, mirroring the rest of the API:
//
// * The value-class tier (`session_*zid`, no `z_` prefix) returns the `ZenohId`
//   data twin / `Vec<ZenohId>`. With the unified newtype projection, value
//   classes ride the same fold-and-wrap machinery as opaque handles: the
//   generated Kotlin surface is `ZenohId` / `List<ZenohId>`, each value erased
//   to its inner `[B` over the wire and wrapped on the Kotlin side, with no
//   raw-bytes hop visible in the FFI surface. This tier targets the JNI adapter.
// * The opaque-handle tier (`z_session_*zid`) returns the `ZZenohId` handle
//   directly (`zenoh::session::ZenohId`), exported by the C layer; callers then
//   use `z_zenoh_id_to_string` / `z_zenoh_id_to_bytes`.
#[prebindgen]
pub fn session_zid(session: &ZSession) -> ZenohId {
    ZenohId::from(session.info().zid().wait())
}

#[prebindgen]
pub fn session_peers_zid(session: &ZSession) -> Vec<ZenohId> {
    session
        .info()
        .peers_zid()
        .wait()
        .map(ZenohId::from)
        .collect()
}

#[prebindgen]
pub fn session_routers_zid(session: &ZSession) -> Vec<ZenohId> {
    session
        .info()
        .routers_zid()
        .wait()
        .map(ZenohId::from)
        .collect()
}
