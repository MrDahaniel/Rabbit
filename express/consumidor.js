const amqp = require("amqplib/callback_api");

amqp.connect("amqp://localhost:3030", (err, connection) => {
	if (err) {
		throw err;
	}
	connection.createChannel((err, channel) => {
		let queueName = "node_queue";
		channel.assertQueue(queueName, {
			durable: false,
		});
		channel.consume(
			queueName,
			(msg) => {
				console.log(`Recibido de manera correcta: ${msg.content.toString()}`);
			},
			{
				noAck: true,
			}
		);
	});
});
