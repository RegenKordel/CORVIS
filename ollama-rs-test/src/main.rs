use ollama_rs::{        
        generation::completion::{
            request::GenerationRequest,
        },
    Ollama,
    };
use tokio::io::{stdout, AsyncWriteExt};
use tokio_stream::StreamExt;


#[tokio::main]
async fn main() {
// By default, it will connect to localhost:11434
    println!("Hello World?!");
    let ollama = Ollama::default();
    let model = "gemma3:12b".to_string();
    let prompt = "Why is the sky blue?".to_string();

    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    if let Ok(res) = res {
        println!("{}", res.response);
    }
}