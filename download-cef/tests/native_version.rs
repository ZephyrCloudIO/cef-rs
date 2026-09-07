use download_cef::default_version;
use semver::{Version, VersionReq};

#[test]
fn binding_release_retains_the_custom_native_sdk() {
    let package = "151.8.1+151.1.0-HEAD.3586";
    assert!(VersionReq::parse("=151.8.1")
        .unwrap()
        .matches(&Version::parse(package).unwrap()));
    assert_eq!(default_version(package), "151.1.0-HEAD.3586");
}

#[test]
fn native_version_without_binding_metadata_is_unchanged() {
    assert_eq!(default_version("151.1.0-HEAD.3586"), "151.1.0-HEAD.3586");
}
