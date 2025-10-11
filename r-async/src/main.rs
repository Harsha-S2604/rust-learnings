#[tokio::main]
async fn main() {
	let fut = my_function();
	println!("Running async code in the main");
	fut.await;
}

async fn slow_task() {
	for i in 1..1_000_000_000 {
	}

	println!("DONE");
}


/* it implements the trait future
The below code the sugar coat of this code:
	fn my_function() -> impl Future<Output = ()> {
		async {
			println!("i am an async function!");
		}
	}

trait Future {
	type Output;
	fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll {
	Ready(T),
	Pending,
}

state machine keeps track of this my_function state
*/

async fn my_function() {
	println!("I am an async function!");
	let s1: String = read_from_db().await;
	println!("First result: {s1}");
	let s2: String = read_from_db().await;
	println!("Second result: {s2}");
}

async fn read_from_db() -> String {
	"DB Result".to_owned()
}
