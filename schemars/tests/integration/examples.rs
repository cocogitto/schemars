use crate::prelude::*;

#[derive(Default, JsonSchema, Serialize)]
#[cocogitto_schemars(example = Struct::default(), example = ())]
struct Struct {
    #[cocogitto_schemars(example = 4 + 4, example = ())]
    foo: i32,
    bar: bool,
    #[cocogitto_schemars(example = (), example = &"foo")]
    baz: Option<&'static str>,
}

#[test]
fn examples() {
    test!(Struct).assert_snapshot();
}
