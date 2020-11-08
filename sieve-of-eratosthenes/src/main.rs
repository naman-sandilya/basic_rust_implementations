fn sieve_of_eratosthenes(primes: &mut Vec<i32>, n: i32) {
    // println!("Iteration started for {}", n);

    for i in 0..primes.len() {
        let value = primes[i];
        if value != 0 && value != n {
            if value % n == 0 {
                // println!("{}", primes[i]); // Checking if right values i.e. composite numbers are done zero
                primes[i] = 0;
            }
        }
    }
    // println!("Iteration done for {}", n);
}

fn main() {
    // Retrieving
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please enter a number to find prime numbers upto that number.");
        return;
    }

    // Parsing arguments
    let n: i32 = args[1].parse().unwrap();
    if n < 2 {
        println!("The integer must be greater than or equal to two.")
    }

    // Creating list of  2..n numbers
    let mut primes: Vec<i32> = Vec::new();
    for i in 2..=n {
        primes.push(i);
    }

    for i in 0..primes.len() {
        let n = primes[i];
        if n != 0 {
            sieve_of_eratosthenes(&mut primes, n);
        }
    }

    // Printing all the prime numbers
    for i in 0..primes.len() {
        if primes[i] != 0 {
            println!("{}", primes[i])
        }
    }
}
