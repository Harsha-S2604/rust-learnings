use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
	let (tx, rx) = mpsc::channel();
	let tx1 = tx.clone();

	// return type is JoinHandle<T>
	let nums = vec![1, 2, 3];
	let handle = thread::spawn(|| {
		for i in 1..10 {
			println!("Hi number {i} from the spawned thread");
			thread::sleep(Duration::from_millis(1));
		}
	});

	let handle_2 = thread::spawn(move || {
		println!("The nums are {nums:?}");
	});

	handle.join().unwrap();
	handle_2.join().unwrap();

	for i in 1..5 {
		println!("Hi number {i} from the main thread");
		thread::sleep(Duration::from_millis(1));
	}

	thread::spawn(move || {
		let val = String::from("Hello from thread!");
		tx.send(val).unwrap();
	});

	let recieved = rx.recv().unwrap();
	println!("{recieved}");
	
	thread::spawn(move || {
		let vals = vec![
			String::from("This"),
			String::from("is"),
			String::from("a"),
			String::from("secret"),
			String::from("message"),
		];

		for val in vals {
			tx1.send(val).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});


	for received in rx {
		println!("Got: {received}");
	}
	
	let (tx_a, rx_b) = mpsc::channel();
	let handle_a = thread::spawn(move || {
		println!("[Handle_A]:: I am doing some operation here");
		tx_a.send("Hello from handle_a");
	});

	let handle_b = thread::spawn(move || {
		println!("[Handle B]::I am doing some ops here");
		let msg = rx_b.recv().unwrap();
		println!("The message is \"{msg}\"");
	});

	handle_a.join().unwrap();
	handle_b.join().unwrap();
}
