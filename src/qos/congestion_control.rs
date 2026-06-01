use prebindgen_proc_macro::prebindgen;

/// Congestion control policy used when routing data.
#[prebindgen]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CongestionControl {
    Drop = 0,
    Block = 1,
    BlockFirst = 2,
}

// `zenoh::qos::CongestionControl::BlockFirst` is `#[cfg(feature = "unstable")]`;
// keep our C enum stable (all three variants always) and gate only the mapping.
// Without `unstable`, `BlockFirst` degrades to `Block`.
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
            #[cfg(not(feature = "unstable"))]
            CongestionControl::BlockFirst => zenoh::qos::CongestionControl::Block,
        }
    }
}
