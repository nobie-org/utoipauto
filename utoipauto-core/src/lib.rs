pub mod attribute_utils;
pub mod discover;
pub mod file_utils;
pub mod string_utils;
pub mod token_utils;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cfg_attr_integration() {
        // Create a test file with cfg_attr attributes
        std::env::set_var("WORKSPACE_ROOT", env!("CARGO_MANIFEST_DIR"));

        let test_content = r#"
use utoipa::{ToSchema, ToResponse};

#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct TestSchema {
    pub name: String,
}

#[cfg_attr(feature = "openapi", derive(ToResponse))]  
pub struct TestResponse {
    pub message: String,
}

#[cfg_attr(feature = "openapi", utoipa::path(
    get,
    path = "/test",
    responses(
        (status = 200, description = "Test response", body = TestResponse),
    ),
))]
pub fn test_cfg_attr_endpoint() -> TestResponse {
    TestResponse { message: "test".to_string() }
}

#[utoipa::path(
    get,
    path = "/regular",
    responses(
        (status = 200, description = "Regular response"),
    ),
)]
pub fn regular_endpoint() {}
"#;

        // Write test content to a temporary file
        let test_file = "test_cfg_attr_temp.rs";
        std::fs::write(test_file, test_content).expect("Failed to write test file");

        // Test discovery
        let result = discover::discover_from_file(test_file.to_string(), "crate".to_string());

        // Clean up
        std::fs::remove_file(test_file).ok();

        let (paths, models, responses) = result;

        println!("Found paths: {:?}", paths);
        println!("Found models: {:?}", models);
        println!("Found responses: {:?}", responses);

        // Should find both cfg_attr and regular utoipa::path attributes
        assert!(
            paths.iter().any(|p| p.contains("test_cfg_attr_endpoint")),
            "cfg_attr endpoint not found"
        );
        assert!(
            paths.iter().any(|p| p.contains("regular_endpoint")),
            "regular endpoint not found"
        );

        // Should find cfg_attr wrapped derives
        assert!(
            models.iter().any(|m| m.contains("TestSchema")),
            "cfg_attr schema not found"
        );
        assert!(
            responses.iter().any(|r| r.contains("TestResponse")),
            "cfg_attr response not found"
        );
    }
}
