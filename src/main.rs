use vector_testing::

pub mod vector_testing {
    tonic::include_proto!("Anki.Vector.external_interface");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
