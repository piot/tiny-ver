/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/tiny-ver
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TinyVersion {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    InvalidFormat,
    InvalidNumber,
    InvalidPreRelease,
}

#[derive(Debug, Eq, PartialEq)]
pub enum NameError {
    InvalidName(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum SplitError {
    MissingHyphen,
    VersionParseError(ParseError),
}

impl FromStr for TinyVersion {
    type Err = ParseError;

    /// Parses a version string in the format "major.minor.patch" or "major.minor.patch-pre_release".
    ///
    /// # Examples
    ///
    /// ```
    /// # use tiny_ver::TinyVersion;
    /// let version: TinyVersion = "1.2.3".parse().unwrap();
    /// assert_eq!(version.to_string(), "1.2.3");
    ///
    /// let version: TinyVersion = "1.2.3-beta".parse().unwrap();
    /// assert_eq!(version.to_string(), "1.2.3-beta");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, '-');
        let version_part = parts.next().ok_or(ParseError::InvalidFormat)?;
        let pre_release_part = parts.next();

        let version_parts: Vec<&str> = version_part.split('.').collect();
        if version_parts.len() != 3 {
            return Err(ParseError::InvalidFormat);
        }

        let major = version_parts[0]
            .parse()
            .map_err(|_| ParseError::InvalidNumber)?;
        let minor = version_parts[1]
            .parse()
            .map_err(|_| ParseError::InvalidNumber)?;
        let patch = version_parts[2]
            .parse()
            .map_err(|_| ParseError::InvalidNumber)?;

        let pre_release = match pre_release_part {
            Some(s) => {
                // Enforce that the pre-release part is non-empty
                if s.is_empty() {
                    return Err(ParseError::InvalidPreRelease);
                }
                // Split the pre-release part by '.' to get individual identifiers
                let identifiers: Vec<&str> = s.split('.').collect();
                if identifiers.iter().any(|id| id.is_empty()) {
                    return Err(ParseError::InvalidPreRelease);
                }
                for id in identifiers {
                    // Each identifier must contain only ASCII alphanumeric characters or hyphen.
                    if !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
                        return Err(ParseError::InvalidPreRelease);
                    }
                    // If the identifier is numeric, it must not have leading zeros (except for "0").
                    if id.chars().all(|c| c.is_ascii_digit()) && id.len() > 1 && id.starts_with('0')
                    {
                        return Err(ParseError::InvalidPreRelease);
                    }
                }
                Some(s.to_string())
            }
            None => None,
        };

        Ok(Self {
            major,
            minor,
            patch,
            pre_release,
        })
    }
}

impl TinyVersion {
    /// # Errors
    /// Return `NameError` if the name is not conforming to `is_valid_name`.
    pub fn versioned_name(&self, name: &str) -> Result<String, NameError> {
        if !is_valid_name(name) {
            return Err(NameError::InvalidName(name.to_string()));
        }
        let result = self.pre_release.as_ref().map_or_else(
            || format!("{}-{}.{}.{}", name, self.major, self.minor, self.patch),
            |pre| {
                format!(
                    "{}-{}.{}.{}-{}",
                    name, self.major, self.minor, self.patch, pre
                )
            },
        );

        Ok(result)
    }
}

impl fmt::Display for TinyVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.pre_release {
            Some(pre) => write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, pre),
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}

/// Checks if the provided name is valid.
///
/// The name must:
/// - Be non-empty.
/// - Consist solely of lower-case alphabetic characters, with _' used as an optional separator.
/// - Not start or end with a '_'.
#[must_use]
pub fn is_valid_name(name: &str) -> bool {
    let Some(first) = name.chars().next() else {
        return false;
    };

    let Some(last) = name.chars().last() else {
        return false;
    };

    // The first and last characters must be lower-case letters.
    if !first.is_ascii_lowercase() || !last.is_ascii_lowercase() {
        return false;
    }
    // All characters in between must be lower-case letters or underscores.
    name.chars().all(|c| c.is_ascii_lowercase() || c == '_')
}

/// Splits a versioned name into its package name and version.
///
/// # Arguments
///
/// * `full_name` - A string in the format produced by [`TinyVersion::versioned_name`],
///   e.g. `"mypackage-1.2.3"` or `"mypackage-1.2.3-beta"`.
///
/// # Returns
///
/// A tuple containing the package name and the parsed [`TinyVersion`].
///
/// # Errors
///
/// Returns a [`SplitError`] if:
/// - The input does not contain a hyphen (thus, no valid separator between package and version).
/// - The version part cannot be parsed as a valid [`TinyVersion`].
pub fn split_versioned_name(full_name: &str) -> Result<(String, TinyVersion), SplitError> {
    let hyphen_index = full_name.find('-').ok_or(SplitError::MissingHyphen)?;
    let name = &full_name[..hyphen_index];
    let version_str = &full_name[hyphen_index + 1..];
    let version = TinyVersion::from_str(version_str).map_err(SplitError::VersionParseError)?;

    Ok((name.to_string(), version))
}
