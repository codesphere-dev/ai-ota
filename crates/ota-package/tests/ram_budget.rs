//! RAM-budget proof for the `.otapkg` parser.
//!
//! Uses a counting global allocator to assert that parsing a package does not
//! allocate more than the declared budget (16 KiB), so the parser stays
//! suitable for constrained MCUs.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use ota_core::Version;
use ota_package::{Package, PackageBuilder, Quantization};

/// Maximum number of bytes the parser may hold live at once.
const BUDGET_BYTES: usize = 16 * 1024;

static CURRENT: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

struct CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            let now = CURRENT.fetch_add(layout.size(), Ordering::SeqCst) + layout.size();
            PEAK.fetch_max(now, Ordering::SeqCst);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        CURRENT.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static ALLOC: CountingAllocator = CountingAllocator;

fn package_bytes() -> Vec<u8> {
    let payload = vec![0x5a; 4096];
    PackageBuilder::new(
        "ram-budget",
        Version::new(1, 0, 0),
        "generic",
        Quantization::Int8,
    )
    .with_payload(payload)
    .build_unsigned()
    .expect("build")
}

#[test]
fn parse_stays_within_ram_budget() {
    let bytes = package_bytes();
    let before = CURRENT.load(Ordering::SeqCst);
    let package = Package::parse(&bytes).expect("parse");
    let peak = PEAK.load(Ordering::SeqCst);
    let parser_peak = peak.saturating_sub(before);
    assert!(!package.is_signed());
    assert!(
        parser_peak <= BUDGET_BYTES,
        "peak allocation {parser_peak} bytes exceeds budget {BUDGET_BYTES} bytes"
    );
}

#[test]
fn verify_payload_stays_within_ram_budget() {
    let bytes = package_bytes();
    let package = Package::parse(&bytes).expect("parse");
    let before = CURRENT.load(Ordering::SeqCst);
    package.verify_payload().expect("verify");
    let peak = PEAK.load(Ordering::SeqCst);
    let parser_peak = peak.saturating_sub(before);
    assert!(
        parser_peak <= BUDGET_BYTES,
        "peak allocation {parser_peak} bytes exceeds budget {BUDGET_BYTES} bytes"
    );
}
