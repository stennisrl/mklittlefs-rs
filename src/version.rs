pub const VERSION_STRING: &str = concat!(
    env!("VERGEN_BUILD_SEMVER"),
    "-",
    env!("VERGEN_GIT_SHA_SHORT"),
    "-",
    env!("VERGEN_CARGO_TARGET_TRIPLE")
);