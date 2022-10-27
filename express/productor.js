const amqp = require("amqplib/callback_api");

amqp.connect("amqp://localhost", (err, connection) => {
	if (err) {
		throw err;
	}
	connection.createChannel((err, channel) => {
		let queueName = "Cola 2";
		let mensajes = [
			{
				nombre: "mensaje1",
				cuerpo: "Hola mundo mensaje 1",
			},
			{
				nombre: "mensaje2",
				cuerpo: "Hola mundo mensaje 2",
			},
			{
				nombre: "mensaje3",
				cuerpo: "Hola mundo mensaje 3",
			},
			{
				nombre: "mensaje4",
				cuerpo: "Hola mundo mensaje 4",
			},
			{
				nombre: "mensaje5",
				cuerpo: "Hola mundo mensaje 5",
			},
		];
		channel.assertQueue(queueName, {
			durable: false,
		});
		channel.sendToQueue(queueName, Buffer.from(mensajes));
		setTimeout(() => {
			connection.close();
		}, 1000);
	});
});
