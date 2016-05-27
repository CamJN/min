use std::io::BufRead;
use std::str::FromStr;
use std::fmt::Debug;

fn iter_to_min<T,U>(i:T) -> U where T:Iterator<Item=String>,U: Ord+FromStr, U::Err: Debug{
    i.collect::<Vec<String>>()
        .iter()
        .flat_map(|s|s.split_whitespace())
        .map(str::parse::<U>)
        .map(Result::unwrap)
        .min()
        .expect("No min found.")
}

fn main() {
    let a: Vec<_> = std::env::args().skip(1).collect();
    let m:i64 = if a.is_empty() {
        let s = std::io::stdin();
        let m = iter_to_min(s.lock().lines().map(Result::unwrap));
        m
    }else{
        iter_to_min(a.into_iter())
    };
    println!("{}", m);
}
