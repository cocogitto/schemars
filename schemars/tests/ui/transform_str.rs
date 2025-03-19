use cocogitto_schemars::JsonSchema;

#[derive(JsonSchema)]
#[cocogitto_schemars(transform = "x")]
pub struct Struct;

fn main() {}
