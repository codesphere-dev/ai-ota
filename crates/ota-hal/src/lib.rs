//! Hardware abstraction layer for flash, storage, RNG and watchdog.
//!
//! The crate is `no_std` so it can be linked into the MCU-side crates.

#![no_std]

/// Capabilities of a flash device.
///
/// A target's flash reports these properties so the bootloader and agent can
/// decide how to stage and commit a package without assuming a particular
/// silicon.
pub trait FlashCapabilities {
    /// Size in bytes of one erase block (the smallest unit that can be erased).
    fn erase_unit(&self) -> usize;

    /// Size in bytes of one write page (the smallest unit that can be written).
    fn page_unit(&self) -> usize;

    /// Whether the flash supports execute-in-place (XIP).
    fn xip(&self) -> bool;

    /// Maximum time, in milliseconds, for a single write operation.
    fn max_write_time_ms(&self) -> u32;

    /// Whether writes can be performed in place (without an erase first).
    fn in_place(&self) -> bool;

    /// Total capacity of the flash in bytes.
    fn capacity(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::FlashCapabilities;

    struct MockFlash;

    impl FlashCapabilities for MockFlash {
        fn erase_unit(&self) -> usize {
            4096
        }
        fn page_unit(&self) -> usize {
            256
        }
        fn xip(&self) -> bool {
            true
        }
        fn max_write_time_ms(&self) -> u32 {
            5
        }
        fn in_place(&self) -> bool {
            false
        }
        fn capacity(&self) -> usize {
            1024 * 1024
        }
    }

    #[test]
    fn reports_capabilities() {
        let flash = MockFlash;
        assert_eq!(flash.erase_unit(), 4096);
        assert_eq!(flash.page_unit(), 256);
        assert!(flash.xip());
        assert_eq!(flash.max_write_time_ms(), 5);
        assert!(!flash.in_place());
        assert_eq!(flash.capacity(), 1024 * 1024);
    }
}