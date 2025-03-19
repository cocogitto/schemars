use cog_schemars::{schema_for, JsonSchema, Schema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
#[cog_schemars(rename_all = "camelCase", deny_unknown_fields, extend("x-customProperty" = "example"))]
pub struct MyStruct {
    #[serde(rename = "thisIsOverridden")]
    #[cog_schemars(rename = "myNumber", range(min = 1, max = 10), transform = remove_format)]
    pub my_int: i32,
    pub my_bool: bool,
    #[cog_schemars(default)]
    pub my_nullable_enum: Option<MyEnum>,
    #[cog_schemars(inner(regex(pattern = "^x$")))]
    pub my_vec_str: Vec<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[cog_schemars(untagged)]
pub enum MyEnum {
    StringNewType(#[cog_schemars(email)] String),
    StructVariant {
        #[cog_schemars(length(min = 1, max = 100))]
        floats: Vec<f32>,
    },
}

fn remove_format(schema: &mut Schema) {
    schema.remove("format");
}

fn main() {
    let schema = schema_for!(MyStruct);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
