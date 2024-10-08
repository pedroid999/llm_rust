use langchain_rust::{language_models::llm::LLM, llm::ollama::client::Ollama};

#[tokio::main]
async fn main() {
    let ollama = Ollama::default().with_model("llama3.1:8b");

    let response = ollama.invoke("Quien descubrió América?").await.unwrap();
    println!("{}", response);
}
