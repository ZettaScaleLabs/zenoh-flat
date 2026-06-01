use crate::util::OnceDrop;
use crate::{
    into_native, CongestionControl, ConsolidationMode, Encoding, Error, KeyExpr, Priority, Query,
    QueryTarget, Reply, ReplyKeyExpr, Sample, ZBytes, ZConfig, ZEncoding, ZKeyExpr,
    ZPublisher, ZQuerier, ZQuery, ZQueryable, ZReply, ZSample, ZSession, ZSubscriber, ZZBytes,
    ZZenohId, ZenohId,
};
// `Reliability` is unstable-only (mirrors `zenoh::qos::Reliability`).
#[cfg(feature = "unstable")]
use crate::Reliability;
use prebindgen_proc_macro::prebindgen;
use std::time::Duration;
use zenoh::{query::Selector, Wait};

/// Open a session with the given configuration. The config is consumed by value
/// (matching native `zenoh::open`); C callers that need to keep it should
/// `z_config_clone` first.
#[prebindgen]
pub fn z_open(config: ZConfig) -> Result<ZSession, Error> {
    zenoh::open(config).wait().map_err(Error::from)
}

// The `reliability` QoS is unstable in zenoh; gate the single parameter (and the
// `.reliability()` call) with `#[cfg(feature = "unstable")]`. prebindgen honors
// per-parameter cfg, so the captured signature — and the generated C ABI — gains
// or loses the trailing `reliability` param with the feature, from ONE definition.
#[prebindgen]
pub fn z_session_declare_publisher(
    session: &ZSession,
    key_expr: ZKeyExpr,
    congestion_control: CongestionControl,
    priority: Priority,
    express: bool,
    #[cfg(feature = "unstable")] reliability: Reliability,
) -> Result<ZPublisher, Error> {
    #[allow(unused_mut)]
    let mut builder = session
        .declare_publisher(key_expr)
        .congestion_control(congestion_control.into())
        .priority(priority.into())
        .express(express);
    #[cfg(feature = "unstable")]
    {
        builder = builder.reliability(reliability.into());
    }
    builder.wait().map_err(Error::from)
}

#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn z_session_put(
    session: &ZSession,
    key_expr: &ZKeyExpr,
    payload: ZZBytes,
    encoding: &ZEncoding,
    congestion_control: CongestionControl,
    priority: Priority,
    express: bool,
    attachment: Option<ZZBytes>,
    #[cfg(feature = "unstable")] reliability: Reliability,
) -> Result<(), Error> {
    let mut builder = session
        .put(key_expr, payload)
        .congestion_control(congestion_control.into())
        .encoding(encoding.clone())
        .express(express)
        .priority(priority.into());
    #[cfg(feature = "unstable")]
    {
        builder = builder.reliability(reliability.into());
    }
    if let Some(att) = attachment {
        builder = builder.attachment(att);
    }
    builder.wait().map_err(Error::from)
}

#[prebindgen]
pub fn z_session_delete(
    session: &ZSession,
    key_expr: &ZKeyExpr,
    congestion_control: CongestionControl,
    priority: Priority,
    express: bool,
    attachment: Option<ZZBytes>,
    #[cfg(feature = "unstable")] reliability: Reliability,
) -> Result<(), Error> {
    let mut builder = session
        .delete(key_expr)
        .congestion_control(congestion_control.into())
        .express(express)
        .priority(priority.into());
    #[cfg(feature = "unstable")]
    {
        builder = builder.reliability(reliability.into());
    }
    if let Some(att) = attachment {
        builder = builder.attachment(att);
    }
    builder.wait().map_err(Error::from)
}

/// Declare a subscriber delivering each change as an opaque [`ZSample`] handle
/// (thin surface). `on_close` fires when the subscriber is dropped.
#[prebindgen]
pub fn z_session_declare_subscriber(
    session: &ZSession,
    key_expr: ZKeyExpr,
    callback: impl Fn(ZSample) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<ZSubscriber, Error> {
    let on_close = OnceDrop::new(on_close);
    session
        .declare_subscriber(key_expr)
        .callback(move |sample| {
            let _ = &on_close;
            callback(sample);
        })
        .wait()
        .map_err(Error::from)
}

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
    z_session_declare_subscriber(
        session,
        ke,
        move |zs| callback(Sample::from(&zs)),
        on_close,
    )
}

#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn z_session_declare_querier(
    session: &ZSession,
    key_expr: ZKeyExpr,
    target: QueryTarget,
    consolidation: ConsolidationMode,
    congestion_control: CongestionControl,
    priority: Priority,
    express: bool,
    timeout_ms: i64,
    accept_replies: ReplyKeyExpr,
) -> Result<ZQuerier, Error> {
    let consolidation: zenoh::query::ConsolidationMode = consolidation.into();
    session
        .declare_querier(key_expr)
        .congestion_control(congestion_control.into())
        .consolidation(consolidation)
        .express(express)
        .target(target.into())
        .priority(priority.into())
        .timeout(Duration::from_millis(timeout_ms as u64))
        .accept_replies(accept_replies.into())
        .wait()
        .map_err(Error::from)
}

/// Declare a queryable delivering each query as an opaque [`ZQuery`] handle
/// (thin surface). `on_close` fires when the queryable is dropped.
#[prebindgen]
pub fn z_session_declare_queryable(
    session: &ZSession,
    key_expr: ZKeyExpr,
    complete: bool,
    callback: impl Fn(ZQuery) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<ZQueryable, Error> {
    let on_close = OnceDrop::new(on_close);
    session
        .declare_queryable(key_expr)
        .complete(complete)
        .callback(move |query| {
            let _ = &on_close;
            callback(query);
        })
        .wait()
        .map_err(Error::from)
}

/// Declare a queryable delivering each query as a fully decoded [`Query`] data
/// class (thick surface). See [`z_session_declare_queryable`].
#[prebindgen]
pub fn session_declare_queryable(
    session: &ZSession,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    complete: bool,
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

#[prebindgen]
pub fn z_session_declare_keyexpr(
    session: &ZSession,
    key_expr: String,
) -> Result<ZKeyExpr, Error> {
    session.declare_keyexpr(key_expr).wait().map_err(Error::from)
}

#[prebindgen]
pub fn z_session_undeclare_keyexpr(session: &ZSession, key_expr: ZKeyExpr) -> Result<(), Error> {
    session.undeclare(key_expr).wait().map_err(Error::from)
}

/// Query matching queryables, delivering each reply as an opaque [`ZReply`]
/// handle (thin surface). `on_close` fires when the reply stream ends.
#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn z_session_get(
    session: &ZSession,
    key_expr: &ZKeyExpr,
    parameters: Option<String>,
    timeout_ms: i64,
    target: QueryTarget,
    consolidation: ConsolidationMode,
    accept_replies: ReplyKeyExpr,
    congestion_control: CongestionControl,
    priority: Priority,
    express: bool,
    payload: Option<ZZBytes>,
    encoding: &ZEncoding,
    attachment: Option<ZZBytes>,
    callback: impl Fn(ZReply) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), Error> {
    let consolidation: zenoh::query::ConsolidationMode = consolidation.into();
    let selector = Selector::owned(key_expr, parameters.unwrap_or_default());
    let on_close = OnceDrop::new(on_close);
    let mut builder = session
        .get(selector)
        .congestion_control(congestion_control.into())
        .priority(priority.into())
        .express(express)
        .target(target.into())
        .timeout(Duration::from_millis(timeout_ms as u64))
        .consolidation(consolidation)
        .accept_replies(accept_replies.into());
    if let Some(payload) = payload {
        builder = builder.payload(payload).encoding(encoding.clone());
    }
    if let Some(att) = attachment {
        builder = builder.attachment(att);
    }
    builder
        .callback(move |reply| {
            let _ = &on_close;
            callback(reply);
        })
        .wait()
        .map_err(Error::from)
}

/// Query matching queryables, delivering each reply as a fully decoded
/// [`Reply`] data class (thick surface). See [`z_session_get`].
#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn session_get(
    session: &ZSession,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    parameters: Option<String>,
    timeout_ms: i64,
    target: QueryTarget,
    consolidation: ConsolidationMode,
    accept_replies: ReplyKeyExpr,
    congestion_control: CongestionControl,
    priority: Priority,
    express: bool,
    payload: Option<impl Into<ZBytes> + Send + 'static>,
    encoding: impl Into<Encoding> + Send + 'static,
    attachment: Option<impl Into<ZBytes> + Send + 'static>,
    callback: impl Fn(Reply) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), Error> {
    let ke = into_native(key_expr.into())?;
    let payload = payload.map(|p| ZZBytes::from(p.into()));
    let z_encoding: ZEncoding = encoding.into().try_into()?;
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
        &z_encoding,
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
    congestion_control: CongestionControl,
    priority: Priority,
    express: bool,
    reliability: Reliability,
) -> Result<ZPublisher, Error> {
    let ke = into_native(key_expr.into())?;
    z_session_declare_publisher(session, ke, congestion_control, priority, express, reliability)
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
    encoding: impl Into<Encoding> + Send + 'static,
    congestion_control: CongestionControl,
    priority: Priority,
    express: bool,
    attachment: Option<impl Into<ZBytes> + Send + 'static>,
    reliability: Reliability,
) -> Result<(), Error> {
    let ke = into_native(key_expr.into())?;
    let payload: ZZBytes = payload.into().into();
    let z_encoding: ZEncoding = encoding.into().try_into()?;
    let attachment = attachment.map(|a| ZZBytes::from(a.into()));
    z_session_put(
        session,
        &ke,
        payload,
        &z_encoding,
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
    congestion_control: CongestionControl,
    priority: Priority,
    express: bool,
    attachment: Option<impl Into<ZBytes> + Send + 'static>,
    reliability: Reliability,
) -> Result<(), Error> {
    let ke = into_native(key_expr.into())?;
    let attachment = attachment.map(|a| ZZBytes::from(a.into()));
    z_session_delete(session, &ke, congestion_control, priority, express, attachment, reliability)
}

/// Advanced (ergonomic) twin of [`z_session_declare_querier`]. See [`session_declare_publisher`].
#[prebindgen]
#[allow(clippy::too_many_arguments)]
pub fn session_declare_querier(
    session: &ZSession,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    target: QueryTarget,
    consolidation: ConsolidationMode,
    congestion_control: CongestionControl,
    priority: Priority,
    express: bool,
    timeout_ms: i64,
    accept_replies: ReplyKeyExpr,
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
    session.info().peers_zid().wait().map(ZenohId::from).collect()
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

#[prebindgen]
pub fn z_session_zid(session: &ZSession) -> ZZenohId {
    session.info().zid().wait()
}

#[prebindgen]
pub fn z_session_peers_zid(session: &ZSession) -> Vec<ZZenohId> {
    session.info().peers_zid().wait().collect()
}

#[prebindgen]
pub fn z_session_routers_zid(session: &ZSession) -> Vec<ZZenohId> {
    session.info().routers_zid().wait().collect()
}
