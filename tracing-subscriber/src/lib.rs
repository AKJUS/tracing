//! Utilities for implementing and composing [`tracing`] subscribers.
//!
//! [`tracing`] is a framework for instrumenting Rust programs to collect
//! scoped, structured, and async-aware diagnostics. The [`Collect`] trait
//! represents the functionality necessary to collect this trace data. This
//! crate contains tools for composing subscribers out of smaller units of
//! behaviour, and batteries-included implementations of common subscriber
//! functionality.
//!
//! `tracing-subscriber` is intended for use by both `Collector` authors and
//! application authors using `tracing` to instrument their applications.
//!
//! *Compiler support: [requires `rustc` 1.65+][msrv]*
//!
//! [msrv]: #supported-rust-versions
//!
//! ## Subscribers and Filters
//!
//! The most important component of the `tracing-subscriber` API is the
//! [`Subscribe`] trait, which provides a composable abstraction for building
//! [collector]s. Like the [`Collect`] trait, [`Subscribe`] defines a
//! particular behavior for collecting trace data. Unlike [`Collect`],
//! which implements a *complete* strategy for how trace data is collected,
//! [`Subscribe`] provide *modular* implementations of specific behaviors.
//! Therefore, they can be [composed together] to form a [collector] which is
//! capable of recording traces in a variety of ways. See the [`subscribe` module's
//! documentation][subscribe] for details on using [subscribers].
//!
//! In addition, the [`Filter`] trait defines an interface for filtering what
//! spans and events are recorded by a particular subscriber. This allows different
//! [`Subscribe`] implementationss to handle separate subsets of the trace data
//! emitted by a program. See the [documentation on per-subscriber
//! filtering][psf] for more information on using [`Filter`]s.
//!
//! [`Subscribe`]: crate::subscribe::Subscribe
//! [composed together]: crate::subscribe#composing-subscribers
//! [subscribe]: crate::subscribe
//! [subscribers]: crate::subscribe
//! [`Filter`]: crate::subscribe::Filter
//! [psf]: crate::subscribe#per-subscriber-filtering
//!
//! ## Included Collectors
//!
//! The following [collector]s are provided for application authors:
//!
//! - [`fmt`] - Formats and logs tracing data (requires the `fmt` feature flag)
//!
//! ## Feature Flags
//!
//! - `std`: Enables APIs that depend on the Rust standard library
//!   (enabled by default).
//! - `alloc`: Depend on [`liballoc`] (enabled by "std").
//! - `env-filter`: Enables the [`EnvFilter`] type, which implements filtering
//!   similar to the [`env_logger` crate]. **Requires "std"**.
//! - `fmt`: Enables the [`fmt`] module, which provides a subscriber
//!   implementation for printing formatted representations of trace events.
//!   Enabled by default. **Requires "registry" and "std"**.
//! - `ansi`: Enables `fmt` support for ANSI terminal colors. Enabled by
//!   default.
//! - `registry`: enables the [`registry`] module. Enabled by default.
//!   **Requires "std"**.
//! - `json`: Enables `fmt` support for JSON output. In JSON output, the ANSI
//!   feature does nothing. **Requires "fmt" and "std"**.
//! - `local-time`: Enables local time formatting when using the [`time`
//!   crate]'s timestamp formatters with the `fmt` subscriber.
//!
//! ### Optional Dependencies
//!
//! - [`tracing-log`]: Enables better formatting for events emitted by `log`
//!   macros in the `fmt` subscriber. Enabled by default.
//! - [`time`][`time` crate]: Enables support for using the [`time` crate] for timestamp
//!   formatting in the `fmt` subscriber.
//! - [`smallvec`]: Causes the `EnvFilter` type to use the `smallvec` crate (rather
//!   than `Vec`) as a performance optimization. Enabled by default.
//! - [`parking_lot`]: Use the `parking_lot` crate's `RwLock` implementation
//!   rather than the Rust standard library's implementation.
//!
//! ### `no_std` Support
//!
//! In embedded systems and other bare-metal applications, `tracing` can be
//! used without requiring the Rust standard library, although some features are
//! disabled. Although most of the APIs provided by `tracing-subscriber`, such
//! as [`fmt`] and [`EnvFilter`], require the standard library, some
//! functionality, such as the [`Subscribe`] trait, can still be used in
//! `no_std` environments.
//!
//! The dependency on the standard library is controlled by two crate feature
//! flags, "std", which enables the dependency on [`libstd`], and "alloc", which
//! enables the dependency on [`liballoc`] (and is enabled by the "std"
//! feature). These features are enabled by default, but `no_std` users can
//! disable them using:
//!
//! ```toml
//! # Cargo.toml
//! tracing-subscriber = { version = "0.3", default-features = false }
//! ```
//!
//! Additional APIs are available when [`liballoc`] is available. To enable
//! `liballoc` but not `std`, use:
//!
//! ```toml
//! # Cargo.toml
//! tracing-subscriber = { version = "0.3", default-features = false, features = ["alloc"] }
//! ```
//!
//! ## Supported Rust Versions
//!
//! Tracing is built against the latest stable release. The minimum supported
//! version is 1.65. The current Tracing version is not guaranteed to build on
//! Rust versions earlier than the minimum supported version.
//!
//! Tracing follows the same compiler support policies as the rest of the Tokio
//! project. The current stable Rust compiler and the three most recent minor
//! versions before it will always be supported. For example, if the current
//! stable compiler version is 1.69, the minimum supported version will not be
//! increased past 1.66, three minor versions prior. Increasing the minimum
//! supported compiler version is not considered a semver breaking change as
//! long as doing so complies with this policy.
//!
//! [`fmt`]: mod@fmt
//! [`registry`]: mod@registry
//! [`Collect`]: tracing_core::collect::Collect
//! [collector]: tracing_core::collect::Collect
//! [`EnvFilter`]: filter::EnvFilter
//! [`tracing`]: https://crates.io/crates/tracing
//! [`tracing-log`]: https://crates.io/crates/tracing-log
//! [`smallvec`]: https://crates.io/crates/smallvec
//! [`env_logger` crate]: https://crates.io/crates/env_logger
//! [`parking_lot`]: https://crates.io/crates/parking_lot
//! [`time` crate]: https://crates.io/crates/time
//! [`liballoc`]: https://doc.rust-lang.org/alloc/index.html
//! [`libstd`]: https://doc.rust-lang.org/std/index.html
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/tokio-rs/tracing/master/assets/logo-type.png",
    html_favicon_url = "https://raw.githubusercontent.com/tokio-rs/tracing/master/assets/favicon.ico",
    issue_tracker_base_url = "https://github.com/tokio-rs/tracing/issues/"
)]
#![cfg_attr(
    docsrs,
    // Allows displaying cfgs/feature flags in the documentation.
    feature(doc_cfg),
    // Allows adding traits to RustDoc's list of "notable traits"
    feature(doc_notable_trait),
    // Fail the docs build if any intra-docs links are broken
    deny(rustdoc::broken_intra_doc_links),
)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_interfaces,
    private_bounds,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]
// Using struct update syntax when a struct has no additional fields avoids
// a potential source change if additional fields are added to the struct in the
// future, reducing diff noise. Allow this even though clippy considers it
// "needless".
#![allow(clippy::needless_update)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod macros;

pub mod field;
pub mod filter;
pub mod prelude;
pub mod registry;

pub mod subscribe;
pub mod util;

feature! {
    #![feature = "std"]
    pub mod reload;
    pub(crate) mod sync;
}

feature! {
    #![all(feature = "fmt", feature = "std")]
    pub mod fmt;
    pub use fmt::fmt;
    pub use fmt::Subscriber as FmtSubscriber;
}

feature! {
    #![all(feature = "env-filter", feature = "std")]
    pub use filter::EnvFilter;
}

pub use subscribe::Subscribe;

feature! {
    #![all(feature = "registry", feature = "std")]
    pub use registry::Registry;

    /// Creates a default [`Registry`], a [`Collect`](tracing_core::Collect)
    /// implementation which tracks per-span data and exposes it to
    /// [`Subscribe`]s.
    ///
    /// Returns a default [`Registry`].
    pub fn registry() -> Registry {
        Registry::default()
    }
}

mod sealed {
    pub trait Sealed<A = ()> {}
}
