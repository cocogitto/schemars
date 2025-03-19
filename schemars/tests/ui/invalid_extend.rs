use cocogitto_schemars::JsonSchema;

#[derive(JsonSchema)]
#[cocogitto_schemars(extend(x))]
#[cocogitto_schemars(extend("x"))]
#[cocogitto_schemars(extend("x" = ))]
#[cocogitto_schemars(extend("y" = "ok!", "y" = "duplicated!"), extend("y" = "duplicated!"))]
#[cocogitto_schemars(extend("y" = "duplicated!"))]
pub struct Struct;

fn main() {}
