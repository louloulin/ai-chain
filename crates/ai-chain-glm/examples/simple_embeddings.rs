use ai_chain::traits::Embeddings;
use ai_chain_glm::embeddings::Embeddings as MEmbeddings;
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let embeddings = MEmbeddings::default();
    let embedded_vecs = embeddings
        .embed_texts(vec![
            "This is an amazing way of writing LLM-powered applications".to_string(),
        ])
        .await
        .unwrap();
    println!("Embedded text: {:?}", embedded_vecs[0])
}
