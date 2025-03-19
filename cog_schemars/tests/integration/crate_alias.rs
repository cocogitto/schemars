use crate::prelude::*;
use ::cog_schemars as aliased_cog_schemars;

#[allow(dead_code)]
#[derive(aliased_cog_schemars::JsonSchema, Deserialize, Serialize, Default)]
#[cog_schemars(crate = "aliased_cog_schemars")]
struct MyStruct {
    /// Is it ok with doc comments?
    foo: i32,
    #[cog_schemars(extend("x-test" = "...and extensions?"))]
    bar: bool,
}

#[test]
fn crate_alias() {
    test!(MyStruct)
        .assert_allows_ser_roundtrip_default()
        .assert_matches_de_roundtrip(arbitrary_values());
}
