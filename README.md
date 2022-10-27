# Rabbit

> A mini class project

## How to use:

First, having docker installed, run the following command:

```
docker run -d --hostname broker --name rabbit_man -p 8080:15672 -p 3030:5672 rabbitmq:3.11-management
```

Now, with our broker ready, we can use our models.

### high_tide

```sh
cd high_tide
cargo run
```

### snake_lover

```sh
cd snake_lover
python snake_lover.py
```

### news_boy

```sh
cd news_boy
cargo run -- m "Message"
```

### Node Consumer

```sh
cd express
node consumer.js
```

### Node Publisher

```sh
cd express
node publisher.js
```

## Created by:

-   Daniel Delgado
-   Juan Duarte
