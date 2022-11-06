
// The prime factors of 13195 are 5, 7, 13 and 29.

// What is the largest prime factor of the number 600851475143 ?

fn prime_numbers_until(n: i64) -> Vec<i64>{
    let mut prime_nums: Vec<i64> = vec![2];
    let mut prime: bool;
    for i in 2..n {
        // println!("{:?}",i);
        prime = true;
        // i is the number we are checking if its prime

        // Loop over all previously calculated prime numbers
        // Additional optimisation could be to stop looping once item reaches half of i
        for item in &prime_nums {
            if i % *item == 0 && i != *item {
                prime = false;
            }
        }

        // if i is prime, add to prime_nums
        if prime{
            prime_nums.push(i);
        }

    }

    // arr.push(55);
    return prime_nums;
}

fn main() {

    let mut initial_number: i64 = 600851475143;

    // prime_numbers until 10000 is sufficient for this problem
    let prime_numbers = prime_numbers_until(10000);
    // Need a counter to track what index of prime_numbers is the last factor
    let mut i: usize = 0;

    while initial_number != 1 {
        i = i + 1;
        // If a prime_number is a factor, divide the initial number
        if initial_number % prime_numbers[i] == 0 {
            initial_number = initial_number / prime_numbers[i];
        }
        
    }
    
    println!("Largest prime factor {:?}",prime_numbers[i]);
}
