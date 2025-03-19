#![allow(clippy::approx_constant)]

use crate::prelude::*;

static THREE: f64 = 3.0;

#[allow(dead_code)]
#[derive(JsonSchema)]
#[cocogitto_schemars(extend("obj" = {"array": [null, ()]}))]
#[cocogitto_schemars(extend("3" = THREE), extend("pi" = THREE + 0.14))]
struct Struct {
    #[cocogitto_schemars(extend("foo" = "bar"))]
    value: Value,
    #[cocogitto_schemars(extend("type" = ["number", "string"]))]
    int: i32,
}

#[test]
fn extend_struct() {
    test!(Struct).assert_snapshot().custom(|schema, _| {
        assert_eq!(schema.get("obj"), Some(&json!({ "array": [null, null] })));
        assert_eq!(schema.get("3"), Some(&json!(3.0)));
        assert_eq!(schema.get("pi"), Some(&json!(3.14)));
        assert_eq!(
            schema.as_value().pointer("/properties/value"),
            Some(&json!({ "foo": "bar" }))
        );
        assert_eq!(
            schema.as_value().pointer("/properties/int/type"),
            Some(&json!(["number", "string"]))
        );
    });
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[cocogitto_schemars(extend("obj" = {"array": [null, ()]}))]
#[cocogitto_schemars(extend("3" = THREE), extend("pi" = THREE + 0.14))]
struct TupleStruct(
    #[cocogitto_schemars(extend("foo" = "bar"))] Value,
    #[cocogitto_schemars(extend("type" = ["number", "string"]))] usize,
);

#[test]
fn extend_tuple_struct() {
    test!(TupleStruct).assert_snapshot().custom(|schema, _| {
        assert_eq!(schema.get("obj"), Some(&json!({ "array": [null, null] })));
        assert_eq!(schema.get("3"), Some(&json!(3.0)));
        assert_eq!(schema.get("pi"), Some(&json!(3.14)));
        assert_eq!(
            schema.as_value().pointer("/prefixItems/0"),
            Some(&json!({ "foo": "bar" }))
        );
        assert_eq!(
            schema.as_value().pointer("/prefixItems/1/type"),
            Some(&json!(["number", "string"]))
        );
    });
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[cocogitto_schemars(extend("foo" = "bar"))]
enum ExternalEnum {
    #[cocogitto_schemars(extend("foo" = "bar"))]
    Unit,
    #[cocogitto_schemars(extend("foo" = "bar"))]
    NewType(Value),
    #[cocogitto_schemars(extend("foo" = "bar"))]
    Tuple(i32, bool),
    #[cocogitto_schemars(extend("foo" = "bar"))]
    Struct { i: i32, b: bool },
}

#[test]
fn extend_externally_tagged_enum() {
    test!(ExternalEnum).assert_snapshot().custom(|schema, _| {
        assert_eq!(schema.get("foo"), Some(&json!("bar")));

        for i in 0..4 {
            assert_eq!(
                schema.as_value().pointer(&format!("/oneOf/{i}/foo")),
                Some(&json!("bar"))
            );
        }
    });
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[cocogitto_schemars(tag = "t", extend("foo" = "bar"))]
enum InternalEnum {
    #[cocogitto_schemars(extend("foo" = "bar"))]
    Unit,
    #[cocogitto_schemars(extend("foo" = "bar"))]
    NewType(Value),
    #[cocogitto_schemars(extend("foo" = "bar"))]
    Struct { i: i32, b: bool },
}

#[test]
fn extend_internally_tagged_enum() {
    test!(InternalEnum).assert_snapshot().custom(|schema, _| {
        assert_eq!(schema.get("foo"), Some(&json!("bar")));

        for i in 0..3 {
            assert_eq!(
                schema.as_value().pointer(&format!("/oneOf/{i}/foo")),
                Some(&json!("bar"))
            );
        }
    });
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[cocogitto_schemars(tag = "t", content = "c", extend("foo" = "bar"))]
enum AdjacentEnum {
    #[cocogitto_schemars(extend("foo" = "bar"))]
    Unit,
    #[cocogitto_schemars(extend("foo" = "bar"))]
    NewType(Value),
    #[cocogitto_schemars(extend("foo" = "bar"))]
    Tuple(i32, bool),
    #[cocogitto_schemars(extend("foo" = "bar"))]
    Struct { i: i32, b: bool },
}

#[test]
fn extend_adjacently_tagged_enum() {
    test!(AdjacentEnum).assert_snapshot().custom(|schema, _| {
        assert_eq!(schema.get("foo"), Some(&json!("bar")));

        for i in 0..4 {
            assert_eq!(
                schema.as_value().pointer(&format!("/oneOf/{i}/foo")),
                Some(&json!("bar"))
            );
        }
    });
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[cocogitto_schemars(untagged, extend("foo" = "bar"))]
enum UntaggedEnum {
    #[cocogitto_schemars(extend("foo" = "bar"))]
    Unit,
    #[cocogitto_schemars(extend("foo" = "bar"))]
    NewType(Value),
    #[cocogitto_schemars(extend("foo" = "bar"))]
    Tuple(i32, bool),
    #[cocogitto_schemars(extend("foo" = "bar"))]
    Struct { i: i32, b: bool },
}

#[test]
fn extend_untagged_enum() {
    test!(UntaggedEnum).assert_snapshot().custom(|schema, _| {
        assert_eq!(schema.get("foo"), Some(&json!("bar")));

        for i in 0..4 {
            assert_eq!(
                schema.as_value().pointer(&format!("/anyOf/{i}/foo")),
                Some(&json!("bar"))
            );
        }
    });
}
