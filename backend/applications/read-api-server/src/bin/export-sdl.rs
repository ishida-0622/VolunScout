use anyhow::Result;

use query_infrastructure::resolvers::create_schema_builder;

#[tokio::main]
async fn main() -> Result<()> {
    let schema = create_schema_builder().finish();
    println!("{}", schema.sdl());
    Ok(())
}
