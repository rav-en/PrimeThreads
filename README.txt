A simple prime stress test written in Rust

Firstly id like to say, this is my first time using GitHub, so if ive done something wrong with the upload just notify me if you can and i'll try and fix it.

Rust isnt my first language (limited programming in it so far) and so perhaps it isnt written as well as it should be, again im open to suggestions and insight on how to be a better programmer in Rust, and in general really.

Run program at command prompt level.

**Warning** Be careful of using an insane number of threads on a single CPU. Could cause your Operating System to crash or do undesired things.
Max ive used on an i7 8th gen was 24, which ran fine. However i must state that i am not responsible for any damages that can occur, use at own risk.

To Use:

PrimeThreads.exe [number_of_threads] //the default is 10,000,000 prime numbers which is a great start on stressing multi core processors. 
PrimeThreads.exe [number_of_threads] [total_numbers_to_search] //add custom total numbers to search in

