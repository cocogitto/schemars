use crate::prelude::*;
use ::cocogitto_schemars as aliased_schemars;

#[allow(dead_code)]
#[derive(aliased_schemars::JsonSchema, Deserialize, Serialize, Default)]
#[cocogitto_schemars(crate = "aliased_schemars")]
struct MyStruct {
    /// Is it ok with doc comments?
    foo: i32,
    #[cocogitto_schemars(extend("x-test" = "...and extensions?"))]
    bar: bool,
}

#[test]
fn crate_alias() {
    test!(MyStruct)
        .assert_allows_ser_roundtrip_default()
        .assert_matches_de_roundtrip(arbitrary_values());
}
