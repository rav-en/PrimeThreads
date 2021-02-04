use std::thread;
use std::time::SystemTime;

use std::env;
use std::str::FromStr;

pub fn print_help(){ //help commands
  println!("Examples");
  println!("PrimeThreads [Threads] default number to search for is 10000000");
  println!("PrimeThreads [Threads] [Total_Units]");
}

fn main() {
  
  let arg: Vec<String> = env::args().collect(); //Get all input commands

  if arg.len() == 1{
    print_help();
}
else if arg.len() > 1 && arg.len() <= 3{

    let number_threads = i32::from_str(&arg[1]).unwrap(); //get number of threads
    

    let mut handles = vec![];
    let mut numbers_to_search = 10000000; //default value if none is given when executed
    
    if arg.len() == 3{
        numbers_to_search = i32::from_str(&arg[2]).unwrap(); //get numbers to search from cmd input
    }

    if numbers_to_search == 0{
        numbers_to_search = 10000000;
    }

    let start_timer = SystemTime::now();


    for i in 0..number_threads {

        println!("T: {} Started.", i);
        if number_threads == i+1{
            println!();
        }


        // create the thread
        let handle = thread::spawn(move|| {

            let mut n = 2;

            for p in 2..(numbers_to_search/number_threads){ //divide total work among all threads
                for k in 2..(numbers_to_search/number_threads){
                if p % k == 0{
                    break;
                }
                n = p;
                }
                if p == n{
                //println!("t{} found prime {}", i, p);
                }  
                
                if p >= (numbers_to_search/number_threads)-1{

                let end_timer = SystemTime::now();
                
                let result = end_timer.duration_since(start_timer);

                println!("Thread:{} completed: {:? }",i, result );
                }
            }

        });

        // push the handle into the handles
        // vector so we can join them
        handles.push(handle);
    }


    // join the handles in the vector
    for i in handles {
        i.join().unwrap();
    }

    let end_timer = SystemTime::now();

    let result = end_timer.duration_since(start_timer);

    println!();
    println!("total time: {:?}", result );
    
    }else{
        print_help();
    }

}
