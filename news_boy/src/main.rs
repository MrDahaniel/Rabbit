use lapin::{
    options::*, publisher_confirm::Confirmation, BasicProperties, Connection, ConnectionProperties,
};
use tracing::info;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:3030/%2f".into());

    async_global_executor::block_on(async {
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .expect("connection error");

        info!("CONNECTED");

        {
            //send channel
            let channel_a = conn.create_channel().await.expect("create_channel");
            //receive channel
            info!(state=?conn.status().state());

            info!(state=?conn.status().state());

            info!("will publish");
            let payload = b"Banana world!";
            let confirm = channel_a
                .basic_publish(
                    "",
                    "hello",
                    BasicPublishOptions::default(),
                    payload,
                    BasicProperties::default(),
                )
                .await
                .expect("basic_publish")
                .await
                .expect("publisher-confirms");
            assert_eq!(confirm, Confirmation::NotRequested);
            info!(state=?conn.status().state());
        }
    });
}
