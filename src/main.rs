use std::io::BufRead;
use std::str::FromStr;
use std::fmt::Debug;

fn iter_to_min<T,U>(i:T) -> U where T:Iterator<Item=String>,U: Ord+FromStr, U::Err: Debug{
    i.map(|s| {
        s.split_whitespace()
            .map(str::parse::<U>)
            .map(Result::unwrap)
            .min()
            .expect("No min found.")
    }).min().expect("No min found.")
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


#[cfg(test)]
mod tests {
    use super::iter_to_min;

    #[test]
    fn single_line() {
        let a = vec!["0 1 2".to_string()];
        assert_eq!(0, iter_to_min(a.into_iter()));
    }

    #[test]
    fn two_lines() {
        let a = vec!["3 4 5 6 7 ".to_string(), "0 1 2".to_string()];
        assert_eq!(0, iter_to_min(a.into_iter()));
    }

    #[test]
    fn negatives() {
        let a = vec!["-1".to_string(), " 3 4 5 6 7 ".to_string(), "0 1 2".to_string()];
        assert_eq!(-1, iter_to_min(a.into_iter()));
    }

    #[test]
    fn all_negatives() {
        let a = vec!["-1".to_string(), " -3 -4 -5 -6 -7 ".to_string(), "0 -1 -2".to_string()];
        assert_eq!(-7, iter_to_min(a.into_iter()));
    }
}
