#[cfg(feature = "unstable")]
use crate::Reliability;
use crate::util::OnceDrop;
use crate::{
    CongestionControl, ConsolidationMode, Error, Priority, QueryTarget, ReplyKeyExpr, ZConfig,
    ZEncoding, ZKeyExpr, ZPublisher, ZQuerier, ZQuery, ZQueryable, ZReply, ZSample, ZSession,
    ZSubscriber, ZZBytes, ZZenohId,
};
use prebindgen_proc_macro::prebindgen;
use std::time::Duration;
use zenoh::{Wait, query::Selector};

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

#[prebindgen]
pub fn z_session_declare_keyexpr(session: &ZSession, key_expr: String) -> Result<ZKeyExpr, Error> {
    session
        .declare_keyexpr(key_expr)
        .wait()
        .map_err(Error::from)
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
