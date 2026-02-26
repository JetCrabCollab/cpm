pub const STANDALONE_TEMPLATE: &str = r#"
use jetcrab::JetCrabRuntime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = JetCrabRuntime::new();
    
    // The JavaScript code is injected here by cpm bundle
    let app_code = include_str!("app.js");
    
    runtime.evaluate_code(app_code).await?;
    
    Ok(())
}
"#;

pub const CARGO_TOML_TEMPLATE: &str = r#"
[package]
name = "standalone-app"
version = "0.1.0"
edition = "2021"

[dependencies]
jetcrab = { path = "JETCRAB_PATH" }
tokio = { version = "1.0", features = ["full"] }
"#;
