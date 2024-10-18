use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: Arc<Mutex<mpsc::Sender<u32>>>) -> Vec<thread::JoinHandle<()>> {
    let qc = Arc::new(q);
    let mut handles = vec![];

    // First thread for the first_half
    let qc1 = Arc::clone(&qc);
    let tx1 = Arc::clone(&tx);
    let handle1 = thread::spawn(move || {
        for val in &qc1.first_half {
            let tx = tx1.lock().unwrap();
            println!("sending {:?}", val);
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    handles.push(handle1);

    // Second thread for the second_half
    let qc2 = Arc::clone(&qc);
    let tx2 = Arc::clone(&tx);
    let handle2 = thread::spawn(move || {
        for val in &qc2.second_half {
            let tx = tx2.lock().unwrap();
            println!("sending {:?}", val);
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    handles.push(handle2);

    handles
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    let tx = Arc::new(Mutex::new(tx));
    let handles = send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    // Wait for the sender threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}
