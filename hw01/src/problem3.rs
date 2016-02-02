/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = (2..n).collect();
    
    for i in 2..n {
    	if primes.contains(&i) {
    		for j in 2..(n/i) {
    			match primes.binary_search(&(j * i)) {
    				Ok(index) => {primes.remove(index);},
    				Err(_) => ()
    			}
    		}
    	}
    }
    
    primes
}