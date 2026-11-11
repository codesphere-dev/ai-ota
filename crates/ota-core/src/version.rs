use alloc::string::{String, ToString};
use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

use crate::error::Error;

/// A version of the form `MAJOR.MINOR.PATCH[-PRERELEASE]`.
///
/// The ordering follows the semver rule that a prerelease sorts *before* the
/// matching release: `1.2.3-rc.1 < 1.2.3`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    pre: Option<String>,
}

impl Version {
    /// Build a release version without a prerelease tag.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: None,
        }
    }

    /// Build a version with a prerelease tag (for example `rc.1`).
    #[must_use]
    pub fn with_pre(major: u32, minor: u32, patch: u32, pre: impl Into<String>) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: Some(pre.into()),
        }
    }

    /// Major component.
    #[must_use]
    pub const fn major(&self) -> u32 {
        self.major
    }

    /// Minor component.
    #[must_use]
    pub const fn minor(&self) -> u32 {
        self.minor
    }

    /// Patch component.
    #[must_use]
    pub const fn patch(&self) -> u32 {
        self.patch
    }

    /// Prerelease tag, if any.
    #[must_use]
    pub fn pre(&self) -> Option<&str> {
        self.pre.as_deref()
    }

    /// `true` when the version carries a prerelease tag.
    #[must_use]
    pub const fn is_prerelease(&self) -> bool {
        self.pre.is_some()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(pre) = &self.pre {
            write!(f, "-{pre}")?;
        }
        Ok(())
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.patch.cmp(&other.patch))
            .then_with(|| match (&self.pre, &other.pre) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(a), Some(b)) => a.cmp(b),
            })
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let invalid = || Error::InvalidVersion(input.to_string());

        let (core, pre) = match input.split_once('-') {
            Some((core, pre)) => {
                if pre.is_empty() {
                    return Err(invalid());
                }
                (core, Some(pre))
            }
            None => (input, None),
        };

        let mut parts = core.split('.');
        let major = parse_component(parts.next(), input)?;
        let minor = parse_component(parts.next(), input)?;
        let patch = parse_component(parts.next(), input)?;
        if parts.next().is_some() {
            return Err(invalid());
        }

        Ok(Self {
            major,
            minor,
            patch,
            pre: pre.map(ToString::to_string),
        })
    }
}

fn parse_component(part: Option<&str>, input: &str) -> Result<u32, Error> {
    let Some(part) = part else {
        return Err(Error::InvalidVersion(input.to_string()));
    };
    if part.is_empty() || !part.bytes().all(|b| b.is_ascii_digit()) {
        return Err(Error::InvalidVersion(input.to_string()));
    }
    part.parse::<u32>()
        .map_err(|_| Error::InvalidVersion(input.to_string()))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    fn v(s: &str) -> Version {
        s.parse().unwrap()
    }

    #[test]
    fn parses_plain_version() {
        let version = v("1.2.3");
        assert_eq!(
            (version.major(), version.minor(), version.patch()),
            (1, 2, 3)
        );
        assert!(!version.is_prerelease());
    }

    #[test]
    fn parses_prerelease() {
        let version = v("0.4.0-rc.1");
        assert_eq!(version.pre(), Some("rc.1"));
        assert!(version.is_prerelease());
    }

    #[test]
    fn round_trips_through_display() {
        for s in ["1.2.3", "0.0.0", "10.20.30-rc.1", "1.0.0-alpha"] {
            assert_eq!(v(s).to_string(), s);
        }
    }

    #[test]
    fn rejects_malformed_versions() {
        for s in [
            "", "1", "1.2", "1.2.3.4", "1.2.x", "1.2.3-", "a.b.c", "1..2",
        ] {
            assert!(
                s.parse::<Version>().is_err(),
                "expected {s:?} to be rejected"
            );
        }
    }

    #[test]
    fn release_sorts_after_prerelease() {
        assert!(v("1.2.3") > v("1.2.3-rc.1"));
        assert!(v("1.2.4") > v("1.2.3"));
        assert!(v("1.3.0") > v("1.2.9"));
        assert!(v("2.0.0") > v("1.99.99"));
    }
}
