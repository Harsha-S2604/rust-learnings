use std::mem::drop;

fn run_callback<F>(mut callback: F) 
where
	F: FnMut(i32),
{
	callback(10);
}

fn main() {
	let hello_world_closure = || println!("Hello World");
	let string_closure = || -> String { String::from("String World") };
	let add_one_closure = |x: i32| -> i32 { x + 1 }; 
	
	//capturing
	let mut factor = 10;
	let multiply = |x: i32| -> i32 { x * factor };
	
	let mut count = 0;
	let mut inc_count = || {
		count += 1;
	};

	let mut c = 22;
	let mut inc_c = |num: i32| {
		c += num;
	};


	println!("calling string_closure... {}", string_closure());
	println!("add one closure... {}", add_one_closure(12));
	println!("Multiply closure... {}", multiply(12));
	
	inc_count();
	inc_count();
	run_callback(inc_c);
	let _count_rebrowwed = &mut count;
	println!("The count value after increasing is {}", count);
	println!("the value of c after increasing is {}", c);
	let box_var = Box::new(12);

	let consume = || {
		println!("I am going to drop the value {box_var}");
		drop(box_var);
	};

	consume();
}
