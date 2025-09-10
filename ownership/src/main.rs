fn take_str_ownership(s: String) {
  println!("{s}");  
}

fn main() {
    /*
     * Do u guys know how much memory gets allocated when u run a program?
     * Here is the brief explanation of it
     *
     * For example:
     *  int main() {
     *      int c = 12;
     *  }
     *
     *  The above one is a C program, when u compile that program, the
     *  variable c gets allocated 4 bytes. Ok, but why 4 bytes?
     *
     *  That's an excellent question mate. This is where C language design +
     *  CPU architecture+history meet
     *
     *  You see in C standard rule for int, doesn't fit exact sizes, it gives
     *  minimum ranges.
     *  short >= 16 bits
     *  int >= same size as short(16 bits)
     *  long >= 32 bits
     *  long long >= 64 bits
     *
     *  So an int could be 2 bytes or 4 bytes or more, depending on the 
     *  architecture of the system
     *
     *  word = register size of the CPU // for my information
     *
     *  Historical Context:
     *  In early 70s machines like PDP-11 were able to handle 16-bits,
     *      word size = 16 bits
     *      So int = 2 bytes
     *  In 80s - 90s, 32 bit CPUs became standard
     *      word size = 32 bits
     *      So int = 4 bytes
     *  Today (most systems are in 64 bit architecture)
     *      int is still 4 bytes (for backward compatibility)
     *      but long or long long hanmdle larger ranges
     *
     *  Why 4 bytes on Modern Systems?
     *  There are certain factors:
     *  1. Efficiency:
     *      working on 2 byte-value is less efficient than 4 byte values
     *  2. Portability:
     *      C programs written in 80s/90s expect int to be 32 bits. Chaning it
     *      ti 16 would break tons of code.
     *  
     *  Think of int as the natuarl number size for the machine
     *
     *  -> on an old 16-bit machine that was 2 bytes
     *  -> on a modern 32/64 bit machine that's 4 bytes
     *
     *  Now if the above program runs in 32/64-bit architecture. then it
     *  would allocate 4 bytes.
     *
     *  But where would it allocate? Here comes the big guy, THE STACK.
     *
     *  what is stack anyway?
     *
     *  Stack is a memory resides in the RAM, where it helps us to execute
     *  the code.
     *
     *  now who manages the stack?
     *  THE OS. when you start a code, he allocates the chunk of RAM for the
     *  stack to execute the code
     *
     *  Now we know that stack stores the function call frames and local
     *  variables
     *
     *  now from the above C code.
     *      the variable c gets stored in the stack. once the main function is
     *      over, it removes from the stack(clears from the stack).
    *
    *   but why it's getting stored in stack?
    *       basically compiler knows the size of that variable and it
    *       proceeds to store it in the stack
    *   what about the variables that are of unknown size? For example getting
    *   the user input. Well we have solution for that also
    *
    *   We basically store unknow size in heap.
    *
    *   So here is a rule of thumb for you
    *   1. compile-time known size -> stored on the stack.
    *   2. Runtime-determined size -> allocated on the heap.
    *
    *   what is heap? who manages it?
    *       Heap is basically something managed by you. It lives on RAM. You
    *       use C's malloc to allocate some memory on the heap.
    *
    *       It doesn't automatically grow/shrink with function calls--you
    *       manage it manually
    *
    *   Now heap looks interesting and everything is under our control
    *   We are the boss. But there are certain problems we need to address
    *   For example:
    *
    *   Memory leak -> Once u allocate the memory using malloc, you must
    *   free it when you are done with it. or else memory leaks will happen.
    *
    *   Dangling pointers -> once you freed the memory, we might unknowingly
    *   use the same memory again, which will lead to the error
    *
    *   Buffer overflow ->
    *   int main() {
    *        char buf[5];
    *        strcpy(buf, "hello all");
    *    }
    *    The above program leads to buffer overflow because the char can
    *    hold only 5 characters but we are copying the string which is
    *    greater than 5 characters
    *
    *    use-after-free and double free ->
    *    there are chances we might double free the same pointer which leads
    *    to undefined behavior
    *
    *    int main() {
    *       int* p = malloc(5 * sizeof(int));
    *       free(*p);
    *       free(*p);
    *   }
    *
    *   Because of all the above factors, C is considered is not memory
    *   safe.
    *
    *   But there are some languages, where they have runtime garbage collector
    *   we can code without worrying about all the above factors. For example:
    *   Java has a inbuilt garbage collector, that manages memory. It will
    *   check for any memory that has not been free yet then it will
    *   clean up that memory.
    *
    *   But RUST works differently, they don't have garbage collector. They
    *   have ownership, where they work at compile level. But why they don't
    *   have garbage collector
    *
    *   1. Performance: no GC pause or background thread scanning memory.
    *   2. Predicability: memory is freed exactly when the variable goes
    *   out of the scope
    *   3. Low overhead: no extra runtime system needed.
    *
    *   But there are crates like gc if you really want GC semantics.
    *
    *   what is ownership?
    *   a set of rules how a rust program manages memory. if any of the rules
    *   are violated then it won't compile.
    *
    *   when you work on ownership you need to think about stack and heap
    *   where most of the programs won't require you to think about it. 
    *   You will know soon.
    *
    *   OWNERSHIP RULES:
    *       1. Each value in Rust has an owner
    *       2. There can be only one owner at a time
    *       3. when the owner goes out of scope, the value will be dropped
    *
    */


    // in the above program, when s comes into scope it is valid. It remains
    // valid until it goes out of scope.

    /* now to know ownership know better, we need to work on heap. Well, why
     * not heap? As we discussed before, stack are known sizes and the stack
     * will clean up automatically but in heap, everything is in our control
     * we need to clean up manually
     *
    */
    
    // here we are String, a second type of string in rust. which will
    // get allocated in the heap.
    
    {
        let s = String::from("Hello all!");
    }

    /*  in the above code when s goes out of the scope, rust uses drop
        method internally to return the memory. Now s is no longer valid 

        but in other languages like C, we have to use free method to return
        the memory

        and in languages like Java they have Garbage collection internally.

    .*/

    // Variables and data interacting with move
    let x = 5;
    let y = x;
    // in the above code, first we declare x and bind it to 5 and then we
    // declare y and make a copy of x which is again binded to 5. These two
    // values are pushed to stack because there are simple values with a known
    // fixed size.

    let s1 = String::from("hello");
    let s2 = s1;
    /*
     * We might assume here that second line would make a copy of s1 and bind
     * it to s2 but this is not what happens
     *
     * internally the variable s1 has pointer, length and capacity which
     * is grouped together and stored on the stack
     * pointer -> points the "hello" memory address which is on the stack
     * length -> the length of the string
     * capacity -> how much size it can hold in bytes
     *
     * when we assign s1 to s2, we copy the pointer, the length and the
     * capacity that are on the stack. We do not copy the data on the heap.
     * if it copies the data on the heap, it could be very expensive in terms
     * of runtime performance if the data on the heap were large.
     *
     * but what happens when s1 and s2 goes out of the scope. wouldn't they
     * try to free up the same memory. This is known as double free error and
     * is one of the memory safety bugs.
     *
     * So after the line let s2 = s1;. Rust considers s1 as no longer valid.
     * Therefore rust doesn't need to free anything when s1 goes out of scope.
     * Remember the rule each value should have exactly one owner at a time.
     *
     * now here when I try to print s1 it will throw an error
    */
    
    /*
     * when u assign a completely new value to an existing variable. Rust will
     * call drop and free the original value's memory immediately.
    */

    let mut hello_str = String::from("hello");
    take_str_ownership(hello_str);
    // the above program we are moving the ownership by passing as params.
    
    let num = 12;
    make_num_copy(num);
    println!("{num}");
    // the above variable makes the copy as the size is known at compile time
    // because it is easy to copy from the stack
    //

    let hello_string = return_ownership();
    println!("got the ownership: {hello_string}");
    
    let rdr = String::from("rdr2");
    let rdr_got = give_and_get(rdr);
    println!("rdr_got,{rdr_got}");

    let name = String::from("JOHN");
    let (name_str, name_len) = calculate_length(name);

    println!("The name is {name_str} and the length of the name is {name_len}");    
    // but isn't it too much just to get the length of the name, we are passing
    // the name value and returning it with tuple. To resolve this we
    // are gonna use references

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn return_ownership() -> String {
    let s = String::from("hello");
    s
}

fn give_and_get(s: String) -> String {
    println!("inside the give and get value: {s}");
    s
}

fn make_num_copy(num: i32) {
    println!("{num}");
}
