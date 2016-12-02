extern crate crypto;

use std::fs::File;
use std::io::{self, Read};
use std::str::FromStr;

use crypto::digest::Digest;

const LENGTH : usize = 4;
type Hash = crypto::sha1::Sha1;

fn rand() -> [u8; LENGTH] {
    let mut rnd : [u8; LENGTH] = [0; LENGTH];

    let mut f = File::open("/dev/urandom").expect("Unable to open /dev/urandom. please use a normal OS");

    f.read_exact(&mut rnd).expect("Unable read random. please use a normal OS");

    rnd
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
    let my_rand = rand();
    let v = hash(&my_rand);

    // print hash
    let s = to_str(&v);
    println!("You hash is \"{}\". please send it to player B", &s);
    // ask for other guys random
    let answer = ask("Please ask B his random number");

    // xor and print the result and your number
    let ans_v = from_str(&answer);

    if ans_v.len() != my_rand.len() {
        panic!("Length should be {} ", my_rand.len());
    }

    // now to the fun part!
    let mut res_vec = vec_with_size( my_rand.len());

    for i in 0..res_vec.len() {
        res_vec[i] = ans_v[i] ^ my_rand[i];
    }

    // in this lame version we only use one bit.. 
    // but all is ready for better version
    let res = match res_vec[0] & 1 {
        0 => "HEADS",
        1 => "TAILS",
        _ => panic!("bug in rust!")
    };
    println!("Result is {}", res);
    println!("Your original random is \"{}\". Please send it to B is", to_str(&my_rand));

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
    let my_rand = rand();

    println!("Your random number is \"{}\". Please send it to A is", to_str(&my_rand));
    // ask for other guy's number and verify

    let his_rand_ans = ask("Please ask A for his original number");
    let his_rand = from_str(&his_rand_ans);

    let v = hash(&his_rand);

    if v != sig_v {
        panic!("SOMETHING IS WRONG! ABORT!!");
    }
    println!("All checks out.")
}

fn main() {

    match ask("Hello, are you paticipant A or B?").as_ref() {
        "a"|"A" => play_a(),
        "b"|"B" => play_b(),
        _ => panic!("I said choose A OR B !!!")
    }
    
}