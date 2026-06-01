use crate::{
    Error, KeyExpr, Reply, Sample, ZLivelinessToken, ZSession, ZSubscriber, into_native,
    z_liveliness_declare_subscriber, z_liveliness_declare_token, z_liveliness_get,
};
use prebindgen_proc_macro::prebindgen;

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
