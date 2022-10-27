extern crate getopts;
use getopts::Options;
use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties,
};
use tracing::info;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optopt(
        "m",
        "message",
        "Message to be sent to the consumers",
        "MESSAGE",
    );

    opts.optflag("h", "help", "Print this menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:3030/%2f".into());

    async_global_executor::block_on(async {
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .expect("connection error");

        info!(state=?conn.status().state());

        info!("CONNECTED");

        {
            //send channel
            let channel_a = conn.create_channel().await.expect("create_channel");

            //create the queue
            let queue = channel_a
                .queue_declare(
                    "hello",
                    QueueDeclareOptions::default(),
                    FieldTable::default(),
                )
                .await
                .expect("queue_declare");

            info!(state=?conn.status().state());
            info!(?queue, "Declared queue");

            let message = match matches.opt_str("m") {
                Some(msg) => msg,
                None => "No messsage".to_string(),
            };

            info!("will publish");
            let payload = message.as_bytes();

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
        }
    });
}
