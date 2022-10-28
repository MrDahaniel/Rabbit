var amqp = require('amqplib/callback_api');

amqp.connect('amqp://localhost:3030', function (error0, connection) {
	if (error0) {
		throw error0;
	}

	connection.createChannel(function (error1, channel) {
		if (error1) {
			throw error1;
		}

		var node_queue = 'node_queue';
		var rust_queue = 'rs_queue';
		var exchange = 'node_rust';

		channel.assertExchange(exchange, 'fanout', {
			durable: false
		});

		channel.assertQueue(node_queue, {
			durable: false
		});
		channel.assertQueue(rust_queue, {
			durable: false
		});

		channel.bindQueue(node_queue, exchange, "");
		channel.bindQueue(rust_queue, exchange, "");

		var msg = 'Hello from node thing!';

		channel.publish(exchange, '', Buffer.from(msg));
	});

	setTimeout(function () {
		connection.close();
		process.exit(0);
	}, 500);
});

