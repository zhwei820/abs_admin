use lazy_static::lazy_static;
use std::net::SocketAddr;
use serde_json::json;

lazy_static! {
    static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:18081".parse().unwrap();
        volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let req = volo_gen::volo::example::GetItemRequest { id: 1024 };
    let resp = CLIENT.get_item(req).await;
    println!("====>resp = {:?}", resp);

    match resp {
        Ok(info) => tracing::info!("{:?}", info),
        Err(e) => tracing::error!("{:?}", e),
    }
}
