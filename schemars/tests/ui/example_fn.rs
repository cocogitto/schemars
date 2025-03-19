use cocogitto_schemars::JsonSchema;

#[derive(JsonSchema)]
#[cocogitto_schemars(example = "my_fn")]
pub struct Struct;

fn my_fn() {}

fn main() {}
