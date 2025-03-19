use cog_schemars::JsonSchema;

#[derive(JsonSchema)]
#[cog_schemars(transform = "x")]
pub struct Struct;

fn main() {}
