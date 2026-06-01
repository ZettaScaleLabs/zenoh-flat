use prebindgen_proc_macro::prebindgen;

/// Congestion control policy used when routing data.
///
/// `BlockFirst` mirrors `zenoh::qos::CongestionControl::BlockFirst`, which is
/// `#[cfg(feature = "unstable")]`; prebindgen honors per-variant `#[cfg]`, so the
/// generated C enum gains/loses `BlockFirst` with the `unstable` feature.
#[prebindgen]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CongestionControl {
    Drop = 0,
    Block = 1,
    #[cfg(feature = "unstable")]
    BlockFirst = 2,
}

impl From<zenoh::qos::CongestionControl> for CongestionControl {
    fn from(value: zenoh::qos::CongestionControl) -> Self {
        match value {
            zenoh::qos::CongestionControl::Drop => CongestionControl::Drop,
            zenoh::qos::CongestionControl::Block => CongestionControl::Block,
            #[cfg(feature = "unstable")]
            zenoh::qos::CongestionControl::BlockFirst => CongestionControl::BlockFirst,
        }
    }
}

impl From<CongestionControl> for zenoh::qos::CongestionControl {
    fn from(value: CongestionControl) -> Self {
        match value {
            CongestionControl::Drop => zenoh::qos::CongestionControl::Drop,
            CongestionControl::Block => zenoh::qos::CongestionControl::Block,
            #[cfg(feature = "unstable")]
            CongestionControl::BlockFirst => zenoh::qos::CongestionControl::BlockFirst,
        }
    }
}
