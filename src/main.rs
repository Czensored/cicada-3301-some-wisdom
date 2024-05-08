// Made by Isaac Salzman 2024-05-06

// Attempt to solve below page of the Liber Primus from the Cicada 3301 puzzle
// https://static.wikia.nocookie.net/uncovering-cicada/images/6/62/Onion_3_v3.jpg/revision/latest?cb=20140115114029
// Also know as page "Some wisdom" on the Uncovering Cicada Wiki
// https://uncovering-cicada.fandom.com/wiki/How_the_solved_pages_of_the_Liber_Primus_were_solved

// Compile with optimizaiton when using numbers >30: cargo run --release

use phf::phf_map;
use std::fs::File;
use std::io::{self, Write};
use std::time::Instant;

static TO_LETTER_MAP: phf::Map<i32, &'static str> = phf_map! {
    2i32 => "F",
    3i32 => "V",
    5i32 => "TH",
    7i32 => "O",
    11i32 => "R",
    13i32 => "C",
    17i32 => "G",
    19i32 => "W",
    23i32 => "H",
    29i32 => "N",
    31i32 => "I",
    37i32 => "J",
    41i32 => "EO",
    43i32 => "P",
    47i32 => "X",
    53i32 => "S",
    59i32 => "T",
    61i32 => "B",
    67i32 => "E",
    71i32 => "M",
    73i32 => "L",
    79i32 => "NG",
    83i32 => "OE",
    89i32 => "D",
    97i32 => "A",
    101i32 => "AE",
    103i32 => "Y",
    107i32 => "IA",
    109i32 => "EA",
};

#[allow(dead_code)]
fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    let mut divisor = 2;

    while n > 1 {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }
        divisor += 1;
    }

    factors
}
#[allow(dead_code)]
fn _old_get_prime_additions(n: i32, primes: &[i32]) -> Vec<Vec<i32>> {
    let mut print_vec: Vec<Vec<i32>> = Vec::new();
    
    let mut i = 28;
    loop {
        if n - primes[i] >= 0 {
            break;
        }
        if i == 0 || n == 0 {
            break;
        }
        i -= 1;
    }

    let mut small_primes = &primes[..=i];
    let mut small_vec: Vec<i32> = Vec::new();

    while !small_primes.is_empty() {
        let current_prime = small_primes[0];
        if n - current_prime == 0 {
            small_vec.push(current_prime);
            print_vec.push(small_vec.clone());
            small_vec.pop();
        } else if n - current_prime > 0 {
            small_vec.push(current_prime);
            let sub_results = _old_get_prime_additions(n - current_prime, primes);
            for mut sub_result in sub_results {
                sub_result.extend(&small_vec);
                print_vec.push(sub_result);
            }
            small_vec.pop();
        }
        small_primes = &small_primes[1..];
    }

    print_vec
}

#[allow(dead_code)]
fn print_word_combinations(numbers: &Vec<i32>, file: &mut File) -> io::Result<()> {
    for number in numbers {
        if let Some(letter) = TO_LETTER_MAP.get(&number) {
            write!(file, "{}", letter)?;
        }
    }

    writeln!(file)?;
    Ok(())
}

#[allow(dead_code)]
fn nopeity(numbers: &Vec<i32>) {
    for number in numbers {
        if let Some(letter) = TO_LETTER_MAP.get(&number) {
            print!("{}", letter);
        }
    }

    println!();
}

fn get_prime_additions(n: i32, output_file: &mut File, primes: &[i32], small_vec: &mut Vec<i32>) -> io::Result<()> {
    let mut small_primes = primes;

    while !small_primes.is_empty() {
        let current_prime = small_primes[0];
        if n - current_prime == 0 {
            small_vec.push(current_prime);
            if small_vec.len() <= 10 {
                for i in &*small_vec {
                    if let Some(letter) = TO_LETTER_MAP.get(&i) {
                        write!(output_file, "{}", letter)?;
                    }
                }
                writeln!(output_file)?;
            }
            small_vec.pop();
        } else if small_vec.len() <= 10 && n - current_prime > 0 {
            small_vec.push(current_prime); 
            get_prime_additions(n - current_prime, output_file, primes, small_vec)?;
            small_vec.pop();
        }
        small_primes = &small_primes[1..];
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut output_file = File::create("output.txt")?;
    let primes: [i32; 29] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109];
    let mut small_vec: Vec<i32> = Vec::new();

    let start = Instant::now();

    get_prime_additions(30, &mut output_file, &primes, &mut small_vec)?;
    
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);

    Ok(())
}