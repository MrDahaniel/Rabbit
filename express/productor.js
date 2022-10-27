const amqp = require("amqplib/callback_api");

amqp.connect("amqp://localhost:3030", (err, connection) => {
	if (err) {
		throw err;
	}
	connection.createChannel((err, channel) => {
		let queueName = "other-hello";
		let mensaje = "NodeJs!";

		channel.assertQueue(queueName, {
			durable: false,
		});
		channel.sendToQueue(queueName, Buffer.from(mensaje));
		setTimeout(() => {
			connection.close();
		}, 100);
	});
});
