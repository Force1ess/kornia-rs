use tokenizers::Tokenizer;
use anyhow::Error as E;

pub fn get_tokenizer(tokenizer: Option<String>) -> anyhow::Result<Tokenizer> {
    let tokenizer = match tokenizer {
        None => {
            let api = hf_hub::api::sync::Api::new()?;
            let api = api.model("HuggingFaceTB/SmolVLM2-256M-Video-Instruct".to_string());
            api.get("tokenizer.json")?
        }
        Some(file) => file.into(),
    };

    Tokenizer::from_file(tokenizer).map_err(E::msg)
}

fn main() -> anyhow::Result<()> {
    let tokenizer = get_tokenizer(None)?;
    let tokens = tokenizer.encode("Hello, world!", true).unwrap();
    println!("{:?}", tokens);
    Ok(())
}