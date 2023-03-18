use abs_admin::service::CONTEXT;

/// use tokio,because Rbatis specifies the runtime-tokio
#[tokio::main]
async fn main() -> std::io::Result<()> {
    //log
    abs_admin::config::log::init_log();
    //database
    CONTEXT.init_pool().await;
    abs_admin::interface::rest::init_router().await
}
