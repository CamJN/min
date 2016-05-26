use std::io::BufRead;
//use std::str::FromStr;

// fn iter_to_min<T,U>(i:T) -> U where T:Iterator<Item=String>,U: Ord+FromStr{
//     i.flat_map(str::split_whitespace)
//         .map(str::trim)
//         .map(str::parse::<U>)
//         .map(Result::unwrap)
//         .min()
//         .expect("No min found.")
// }

fn iter_to_min<T>(i:T) -> i64 where T:Iterator<Item=String>{
    i.collect::<Vec<String>>()
        .iter()
        .flat_map(|s|s.split_whitespace())
        .map(str::trim)
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .min()
        .expect("No min found.")
}

fn main() {
    let a: Vec<_> = std::env::args().skip(1).collect();
    let m:i64 = if a.is_empty() {
        let i = std::io::stdin();
        let m = iter_to_min(i.lock().lines().map(Result::unwrap));
        m
    }else{
        iter_to_min(a.into_iter())
    };
    println!("{}", m);
}
