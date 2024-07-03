// channels are in the mpsc module: multi-producer, single-consumer
use std::{sync::mpsc, thread, time::Duration};

fn main() {
    channel();

    bounded_channel();
}

// channels are a way to send messages between threads
// main methods: .send() to send a message, .recv() to receive a message (which blocks until a message is available)

fn channel() {
    let (sender, receiver) = mpsc::channel::<String>();

    {
        // use .clone() to allow for multiple senders
        let sender = sender.clone();

        std::thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            // will return the message if unable to be sent -- use unwrap to assume this won't happen
            sender
                .send(String::from("Hello from the other side"))
                .unwrap();
        });
    }

    std::thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        // will return the message if unable to be received -- use unwrap to assume this won't happen
        sender
            .clone()
            .send(String::from("Hello again from the other side"))
            .unwrap();
    });

    let message = receiver.recv().unwrap();
    println!("Message received: {}", message);

    let message = receiver.recv().unwrap();
    println!("Message 2 received: {}", message);
}

// what if senders keep sending but receivers don't keep up? -> memory usage grows
// block on the send if the buffer is full

// if the bound size is 0, the channel is a rendezvous channel, meaning the sender and receiver must meet at the same time in order for the send to succeed
fn bounded_channel() {
    let (sender, receiver) = mpsc::sync_channel::<String>(16);

    let sender_clone = sender.clone();

    std::thread::spawn(move || {
        for i in 1..1000 {
            sender_clone.send(format!("Message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in receiver {
        println!("Got: {}", received);
    }

    // rendevous channel
    let (sender, receiver) = mpsc::sync_channel::<String>(0);

    std::thread::spawn(move || {
        sender.send(String::from("Wait for me")).unwrap();

        println!("Message sent");
    });

    std::thread::spawn(move || {
        let message = receiver.recv().unwrap();
        println!("Message received: {}", message);
    });
}
