mod candle;
use candle::modeling::get_model;
use candle::preprocessing::get_tokenizer;
fn main() -> anyhow::Result<()> {
    let tokenizer = get_tokenizer(None)?;
    let tokens = tokenizer.encode("Hello, world!", true).unwrap();
    let _model = get_model(None)?;
    println!("{:?}", tokens);
    Ok(())
}
