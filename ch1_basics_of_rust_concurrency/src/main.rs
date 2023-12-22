use std::thread;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread.");

    t1.join().unwrap();
    t2.join().unwrap();

    /* Here, ownership of numbers is transferred to the newly spawned thread, since we used a move closure.

    If we had not used the move keyword, the closure would have captured numbers by reference.
    This would have resulted in a compiler error, since the new thread might outlive that variable.
    */
    let numbers = vec![1, 2, 3];
    thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();

    let id = thread::current().id();
    println!("This is my thread id: {:?}", id);

    /*
    Getting a value back out of the thread is done by returning it from the closure. This
    return value can be obtained from the Result returned by the join method:
    */

    let numbers = Vec::from_iter(0..=1000);
    /*
        The std::thread::spawn function is actually just a convenient shorthand for
        std::thread::Builder::new().spawn().unwrap().
    */
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        println!("LEN AND SUM {} {}", len, sum);
        sum / len
    });
    let average = t.join().unwrap();
    println!("average: {average}");
    let id = thread::current().id();
    println!("This is my thread id: {:?}", id);



    
    let numbers = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });
}

fn f() {
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread id: {:?}", id);
}
