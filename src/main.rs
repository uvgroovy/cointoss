extern crate crypto;
extern crate num;

use std::mem;

use num::bigint::BigUint;
use num::bigint::ToBigUint;
use num::ToPrimitive;

use std::fs::File;
use std::io::{self, Read};
use std::str::FromStr;

use crypto::digest::Digest;

const SALT_LENGTH : usize = 8;
const NUMBERS : usize = 2;
type Hash = crypto::sha1::Sha1;

fn rand(bytes : usize) -> BigUint {

    let mut rnd = vec![0; bytes];

    let mut f = File::open("/dev/urandom").expect("Unable to open /dev/urandom. please use a normal OS");

    f.read_exact(&mut rnd).expect("Unable read random. please use a normal OS");

    BigUint::from_bytes_le(&rnd)
}

fn ask(q: &str) -> String {
    println!("{}", q);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read use input");

     String::from_str(input.trim()).unwrap()
}

fn vec_with_size(size: usize) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(0);
    }
    vec
}


fn hash(x : &[u8]) -> Vec<u8> {

    let mut h = Hash::new();

    // to bytes round up
    let mut v = vec_with_size((h.output_bits() + 7) / 8);
    
    h.input(x);
    
    h.result(&mut v);
    v
}

fn play_a() {
    let my_rand = rand(SALT_LENGTH);
    let repr = my_rand.to_bytes_le();
    let v = hash(&repr);

    // print hash
    let s = to_str(&v);
    println!("You hash is \"{}\". please send it to player B", &s);
    // ask for other guys random
    let answer = ask("Please ask B his random number");

    // xor and print the result and your number
    let ans_v = from_str(&answer);

    let his_num = BigUint::from_bytes_le(&ans_v);

    // now to the fun part!
    let res = his_num + my_rand;

    let div = NUMBERS.to_biguint().unwrap();
    // in this lame version we only use one bit.. 
    // but all is ready for better version
    let res = match (res % div).to_usize().unwrap() {
        0 => "HEADS",
        1 => "TAILS",
        _ => panic!("bug in rust!")
    };
    println!("Result is {}", res);
    println!("Your original random is \"{}\". Please send it to B is", to_str(&repr));

}

fn to_str(a : &[u8]) -> String{
    let mut s = String::new();

    for b in a {
        let tmp = format!("{0:02x}", b);
        s.push_str(&tmp);
    }

    s
}
fn from_str(s : &str) -> Vec<u8>{
    let mut v : Vec<u8> =  Vec::new();

    if s.len() % 2 != 0 {
        panic!("REALLY!?")
    }

    for i in 0..(s.len()/2) {
        v.push(u8::from_str_radix(&s[(i*2)..(i*2+2)], 16).expect("stop fooling around!"));
    }

    v
}

fn play_b() {
    // ask for other guy's signature
    let sig_ans = ask("Please ask A for his signature");
    let sig_v = from_str(&sig_ans);

    // print your number
    let div = NUMBERS.to_biguint().unwrap();
    let my_rand = rand(mem::size_of::<usize>()) % &div;

    let repr = my_rand.to_bytes_le();


    println!("Your random number is \"{}\". Please send it to A is", to_str(&repr));
    // ask for other guy's number and verify

    let his_rand_ans = ask("Please ask A for his original number");
    let his_rand_repr = from_str(&his_rand_ans);

    let v = hash(&his_rand_repr);

    if v != sig_v {
        panic!("SOMETHING IS WRONG! ABORT!!");
    }

    let his_rand = BigUint::from_bytes_le(&his_rand_repr);

    // make sure math checks out
    let res = his_rand + my_rand;

    // in this lame version we only use one bit.. 
    // but all is ready for better version
    let res = match (res % div).to_usize().unwrap() {
        0 => "HEADS",
        1 => "TAILS",
        _ => panic!("bug in rust!")
    };
    
    println!("Result is {}", res);
    println!("All checks out.")
}

fn main() {

    match ask("Hello, are you paticipant A or B?").as_ref() {
        "a"|"A" => play_a(),
        "b"|"B" => play_b(),
        _ => panic!("I said choose A OR B !!!")
    }
    
}