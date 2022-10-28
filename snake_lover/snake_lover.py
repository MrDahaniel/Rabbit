from amqp import Connection


with Connection(host="127.0.0.1:3030") as conn:
    channel = conn.channel()

    def on_message(message):
        print(f"Message: {message.body.decode('utf-8')}")

    channel.basic_consume(queue="py_queue", callback=on_message, no_ack=True)

    while True:
        conn.drain_events()
