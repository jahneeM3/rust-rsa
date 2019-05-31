/*

Finding first 2 consecutive integers with distint 2 prime factors is immediate but time increases steeply

$ time cargo run 2 2 1
Sequence { base: 14, factors: {1: [3, 5], 0: [2, 7]}, length: 2 }

real    0m0.026s
user    0m0.020s
sys     0m0.005s

$time cargo run 3 3 1
Sequence { base: 644, factors: {2: [2, 17, 19], 0: [2, 7, 23], 1: [3, 5, 43]}, length: 3 }

real    0m0.062s
user    0m0.040s
sys     0m0.019s

$ time cargo run 4 4 1
Sequence { base: 134043, factors: {1: [2, 23, 31, 47], 0: [3, 7, 13, 491], 3: [2, 3, 11, 677], 2: [5, 17, 19, 83]}, length: 4 }

real    0m2.158s
user    0m2.138s
sys     0m0.016s

Finding first 5 consecutive integers with 5 prime factors has not finished for me after ~15min 

The script could be faster if I only stored the lengths of prime factor vector for each integer, but this is more interesting to inspect
and we're only ever storing 'consec' number of such vectors

Also, u64 is not needed everywhere

*/


use std::env;
use std::collections::HashMap;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() as u8 != 4 {
        panic!("please include: \
               number of consecutive integers, \
               number of distince prime factors required, \
               starting-from integer offset");
    }

    let mut nums: Vec<u64> = Vec::new() ;
    nums.push(args[1].parse().unwrap()); // number of consecutive integers
    nums.push(args[2].parse().unwrap()); // number of distinct prime factors 
    nums.push(args[3].parse().unwrap()); // starting-from integer offset 
    if nums[2] < 1 {
        panic!("only non-zero integer offsets");
    }

    let seq: Sequence = find_sequence(nums[0], nums[1], nums[2]);
    println!("{:?}", seq);
}

#[derive(Debug)]
struct Sequence {
    base: u64,
    factors: HashMap<u64, Vec<u64>>,
    length: u64
}

// find first 'consec' consecutive integers with 'num_primes' number of distinct prime factors
// ex find_sequence(2, 2) = [14,15] 
fn find_sequence(consec: u64, num_primes: u64, offset: u64) -> Sequence {

    // initialize consec number of prime factor vectors
    let mut factors_map: HashMap<u64, Vec<u64>> = HashMap::new();

    let mut seq = Sequence {
       base: offset,
       factors: factors_map,
       length: consec
    };

    for i in 0..consec { 
        seq.factors.insert(i, dist_pfactors(offset + i));
    }

    let mut i: u64 = seq.base + 1;
    let mut consec_matches: bool = true;

    loop { 
        seq.base = i;

        for j in 0..consec {
            
            if j < ( consec - 1 ) {
                // shift factors for j+1 to j
                let handler = seq.factors.remove(&(j+1));
                if let Some(v) = handler {
                    seq.factors.insert(j, v);
                }
            }
            else {
                // compute the next integer's prime factorization  
                let last_consec = dist_pfactors(i + consec - 1);
                
                seq.factors.insert(consec-1, last_consec); 
                consec_matches = true;
                for k in 0..seq.length {

                    let handler = seq.factors.get(&k);
                    if let Some(v) = handler {
                        
                        if v.len() as u64 != num_primes {
                            consec_matches = false;
                            break;
                        }
                    }
                }
           }
        }
        if consec_matches {
            return seq;
        }
        i += 1;
    }

    println!("Cannot find. Returning last sequence analyzed");
    return seq; 
}

// return vector of distinct prime factors of n 
fn dist_pfactors(n: u64) -> Vec<u64> {

    if n == 0 {
        panic!("only nonzero unsigned integers please");
    }

    let mut n_tmp = n;
    let mut pfactors: Vec<u64> = Vec::new();
    
    // remove powers of 2
    while n_tmp % 2 == 0 {
        n_tmp = n_tmp / 2;
        pfactors.push(2);     
    }
   
    // every composite n has prime factor(s) < sqrt(n) 
    let range: u64 = ((n_tmp as f64).sqrt()+1.0) as u64;

    // iterate over odd indices
    for i in (3..range).step_by(2) {
    
        // remove powers of i
        while n_tmp % i == 0 {
            n_tmp = n_tmp / i;
            pfactors.push(i);
        }
    }
    
    if n_tmp > 2 && n_tmp != n {
        pfactors.push(n_tmp);
    }

    // remove duplicates
    pfactors.dedup();
    return pfactors;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn euler47_test() {
        let first_integer = 14;
        let seq: Sequence = find_sequence(2, 2, 1);
        println!("{:?}", seq);
        assert_eq!(
            first_integer,
            seq.base
        );
    }
}
