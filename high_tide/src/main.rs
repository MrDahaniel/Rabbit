use lapin::{
    message::DeliveryResult, options::*, types::FieldTable, Connection, ConnectionProperties,
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
            //receive channels
            let channel_a = conn.create_channel().await.expect("create_channel");
            let channel_b = conn.create_channel().await.expect("create_channel");
            info!(state=?conn.status().state());

            info!("will consume");

            channel_a
                .basic_consume(
                    "other-hello",
                    "my_consumer",
                    BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await
                .expect("basic_consume")
                .set_delegate(move |delivery: DeliveryResult| async move {
                    if let Ok(Some(delivery)) = delivery {
                        delivery
                            .ack(BasicAckOptions::default())
                            .await
                            .expect("basic_ack");

                        if delivery.data.is_ascii() {
                            let data: String =
                                delivery.data.into_iter().map(|x| x as char).collect();
                            info!(message = data, "Message:");
                        }
                    }
                });

            channel_b
                .basic_consume(
                    "hello",
                    "my_consumer",
                    BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await
                .expect("basic_consume")
                .set_delegate(move |delivery: DeliveryResult| async move {
                    // info!(message=?delivery, "received message");
                    if let Ok(Some(delivery)) = delivery {
                        delivery
                            .ack(BasicAckOptions::default())
                            .await
                            .expect("basic_ack");

                        if delivery.data.is_ascii() {
                            let data: String =
                                delivery.data.into_iter().map(|x| x as char).collect();
                            info!(message = data, "Message:");
                        }
                    }
                });
        }

        conn.run().expect("conn.run");
    });
}
