use std::path::PathBuf;

pub fn get_model(model: Option<String>) -> anyhow::Result<PathBuf> {
    let model = match model {
        None => "HuggingFaceTB/SmolVLM2-256M-Video-Instruct".to_string(),
        Some(file) => file.into(),
    };
    let api = hf_hub::api::sync::Api::new()?;
    let api = api.model(model);
    let model = api.get("model.safetensors")?;
    Ok(model)
}
