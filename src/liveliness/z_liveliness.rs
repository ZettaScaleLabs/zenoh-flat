use crate::util::OnceDrop;
use crate::{
    into_native, Error, KeyExpr, Reply, Sample, ZKeyExpr, ZLivelinessToken, ZReply, ZSample,
    ZSession, ZSubscriber,
};
use prebindgen_proc_macro::prebindgen;
use std::time::Duration;
use zenoh::Wait;

/// Declare a [`ZLivelinessToken`] on `key_expr`. The token keeps the liveliness
/// alive until its handle is dropped, which undeclares it.
#[prebindgen]
pub fn z_liveliness_declare_token(
    session: &ZSession,
    key_expr: ZKeyExpr,
) -> Result<ZLivelinessToken, Error> {
    session
        .liveliness()
        .declare_token(key_expr)
        .wait()
        .map_err(Error::from)
}

/// Query liveliness tokens matching `key_expr`, delivering each reply as an
/// opaque [`ZReply`] handle (thin surface — cheap-FFI bindings pull fields via
/// the `z_reply_*` accessors). `on_close` fires when the reply stream ends.
#[prebindgen]
pub fn z_liveliness_get(
    session: &ZSession,
    key_expr: &ZKeyExpr,
    timeout_ms: i64,
    callback: impl Fn(ZReply) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), Error> {
    let on_close = OnceDrop::new(on_close);
    session
        .liveliness()
        .get(key_expr)
        .timeout(Duration::from_millis(timeout_ms as u64))
        .callback(move |reply| {
            let _ = &on_close;
            callback(reply);
        })
        .wait()
        .map_err(Error::from)
}

/// Query liveliness tokens matching `key_expr`, delivering each reply as a
/// fully decoded [`Reply`] data class (thick surface — one FFI hop per reply).
/// See [`z_liveliness_get`] for parameter semantics.
#[prebindgen]
pub fn liveliness_get(
    session: &ZSession,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    timeout_ms: i64,
    callback: impl Fn(Reply) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<(), Error> {
    let ke = into_native(key_expr.into())?;
    z_liveliness_get(
        session,
        &ke,
        timeout_ms,
        move |zr| callback(Reply::from(&zr)),
        on_close,
    )
}

/// Declare a subscriber to liveliness changes matching `key_expr`, delivering
/// each change as an opaque [`ZSample`] handle (thin surface). With `history`,
/// currently-alive tokens are delivered on declaration. `on_close` fires when
/// the returned subscriber is dropped.
#[prebindgen]
pub fn z_liveliness_declare_subscriber(
    session: &ZSession,
    key_expr: ZKeyExpr,
    history: bool,
    callback: impl Fn(ZSample) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<ZSubscriber, Error> {
    let on_close = OnceDrop::new(on_close);
    session
        .liveliness()
        .declare_subscriber(key_expr)
        .history(history)
        .callback(move |sample| {
            let _ = &on_close;
            callback(sample);
        })
        .wait()
        .map_err(Error::from)
}

/// Declare a subscriber to liveliness changes matching `key_expr`, delivering
/// each change as a fully decoded [`Sample`] data class (thick surface). See
/// [`z_liveliness_declare_subscriber`] for parameter semantics.
#[prebindgen]
pub fn liveliness_declare_subscriber(
    session: &ZSession,
    key_expr: impl Into<KeyExpr> + Send + 'static,
    history: bool,
    callback: impl Fn(Sample) + Send + Sync + 'static,
    on_close: impl Fn() + Send + Sync + 'static,
) -> Result<ZSubscriber, Error> {
    let ke = into_native(key_expr.into())?;
    z_liveliness_declare_subscriber(
        session,
        ke,
        history,
        move |zs| callback(Sample::from(&zs)),
        on_close,
    )
}

/// Advanced (ergonomic) twin of [`z_liveliness_declare_token`]: accepts
/// `impl Into<KeyExpr>` and delegates to the explicit `z_` function. Not
/// wrapped by the C adapter; targets the JNI adapter.
#[prebindgen]
pub fn liveliness_declare_token(
    session: &ZSession,
    key_expr: impl Into<KeyExpr> + Send + 'static,
) -> Result<ZLivelinessToken, Error> {
    let ke = into_native(key_expr.into())?;
    z_liveliness_declare_token(session, ke)
}
