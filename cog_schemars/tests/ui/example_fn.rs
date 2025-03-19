use cog_schemars::JsonSchema;

#[derive(JsonSchema)]
#[cog_schemars(example = "my_fn")]
pub struct Struct;

fn my_fn() {}

fn main() {}
