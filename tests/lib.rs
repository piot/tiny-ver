/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/tiny-ver
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use tiny_ver::{is_valid_name, ParseError, TinyVersion};

#[test]
fn add_to_name() {
    assert_eq!(
        "1.2.3-alpha.1"
            .parse::<TinyVersion>()
            .unwrap()
            .versioned_name("myapp")
            .unwrap(),
        "myapp-1.2.3-alpha.1"
    );
}

#[test]
fn invalid_format() {
    let version = "1.2".parse::<TinyVersion>();
    assert_eq!(version, Err(ParseError::InvalidFormat));
}

#[test]
fn invalid_number() {
    let version = "a.2.3".parse::<TinyVersion>();
    assert_eq!(version, Err(ParseError::InvalidNumber));
}

#[test]
fn invalid_pre_release() {
    let version = "1.2.3-!alpha".parse::<TinyVersion>();
    assert_eq!(version, Err(ParseError::InvalidPreRelease));
}

#[test]
fn add_to_name_without_pre_release() {
    let version: TinyVersion = "1.2.3".parse().unwrap();
    let result = version.versioned_name("lib").unwrap();
    assert_eq!(result, "lib-1.2.3");
}

#[test]
fn add_to_name_with_pre_release() {
    let version: TinyVersion = "1.2.3-beta".parse().unwrap();
    let result = version.versioned_name("lib").unwrap();
    assert_eq!(result, "lib-1.2.3-beta");
}

#[test]
fn invalid_numeric_identifier_with_leading_zero() {
    let version = "1.2.3-rc.01".parse::<TinyVersion>();
    assert_eq!(version, Err(ParseError::InvalidPreRelease));
}

#[test]
fn valid_strict_pre_release() {
    let version: TinyVersion = "1.2.3-rc.1".parse().unwrap();
    assert_eq!(version.to_string(), "1.2.3-rc.1");
}

#[test]
fn valid_names() {
    assert!(is_valid_name("foo"));
    assert!(is_valid_name("foo_bar"));
    assert!(is_valid_name("foobar"));
    assert!(is_valid_name("tiny_ver"));
}

#[test]
fn invalid_empty_name() {
    assert!(!is_valid_name(""));
}

#[test]
fn invalid_start_or_end_with_underscore() {
    assert!(!is_valid_name("_foo"));
    assert!(!is_valid_name("foo_"));
}

#[test]
fn invalid_uppercase_letters() {
    assert!(!is_valid_name("Foo"));
    assert!(!is_valid_name("fooBar"));
    assert!(!is_valid_name("FOOBAR"));
}

#[test]
fn invalid_characters() {
    assert!(!is_valid_name("foo-bar"));
    assert!(!is_valid_name("foo1bar"));
    assert!(!is_valid_name("foo!bar"));
}
