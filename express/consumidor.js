const amqp = require("amqplib/callback_api");

amqp.connect("amqp://localhost", (err, connection) => {
	if (err) {
		throw err;
	}
	connection.createChannel((err, channel) => {
		let queueName = "Cola 2";
		channel.assertQueue(queueName, {
			durable: false,
		});
		channel.consume(
			queueName,
			(msg) => {
				console.log(`Recibido de manera correcta: ${msg.content.toString()}`);
				channel.ack(msg);
			},
			{
				noAck: true,
			}
		);
	});
});
