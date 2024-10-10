use langchain_rust::{
    chain::{builder::ConversationalChainBuilder, Chain},
    llm::ollama::client::Ollama,
    memory::SimpleMemory,
    prompt_args,
};
use std::io::{stdout, Write};

use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let ollama = Ollama::default().with_model("llama3.1:8b");
    //We initialize a simple memory. By default conversational chain have this memory,
    //initialize it as an example, if you dont want to have memory use DummyMemory
    let memory = SimpleMemory::new();

    let chain = ConversationalChainBuilder::new()
        .llm(ollama)
        .memory(memory.into())
        .build()
        .expect("Error building ConversationalChain");

    let input_variables = prompt_args! {
        "input"  => "I'm a Rust programmer and I want to build a tauri app with leptos to implement chatbots minimalism to search project documentation",
    };

    let mut stream = chain.stream(input_variables).await.unwrap();
    while let Some(result) = stream.next().await {
        match result {
            Ok(data) => {
                print!("{}", data.content);
                stdout().flush().unwrap();
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    let input_variables = prompt_args! {
        "input"  => "What is leptos?",
    };
    match chain.invoke(input_variables).await {
        Ok(result) => {
            println!("\n)");
            println!("Result {:?}", result);
        }
        Err(e) => panic!("Error invoking LLMChain: {:?}", e),
    }
}
