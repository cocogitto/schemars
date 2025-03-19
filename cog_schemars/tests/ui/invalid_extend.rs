use cog_schemars::JsonSchema;

#[derive(JsonSchema)]
#[cog_schemars(extend(x))]
#[cog_schemars(extend("x"))]
#[cog_schemars(extend("x" = ))]
#[cog_schemars(extend("y" = "ok!", "y" = "duplicated!"), extend("y" = "duplicated!"))]
#[cog_schemars(extend("y" = "duplicated!"))]
pub struct Struct;

fn main() {}
