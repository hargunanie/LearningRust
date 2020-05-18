use rand::Rng;

fn main() {
    println!("Hello world!");
    let randnum = rand::thread_rng().gen_range(1, 1001);
    println!("The random number is: {}", randnum)
}
