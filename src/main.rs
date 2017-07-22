use std::io::BufRead;
use std::str::FromStr;
use std::fmt::Debug;
use std::path::Path;
use std::ffi::OsStr;

fn iter_to_min<T,U>(i:T) -> U where T:Iterator<Item=String>,U: Ord+FromStr, U::Err: Debug{
    i.map(|s| {
        s.split_whitespace()
            .map(str::parse::<U>)
            .map(Result::unwrap)
            .min()
            .expect("No min found.")
    }).min().expect("No min found.")
}
fn iter_to_max<T,U>(i:T) -> U where T:Iterator<Item=String>,U: Ord+FromStr, U::Err: Debug{
    i.map(|s| {
        s.split_whitespace()
            .map(str::parse::<U>)
            .map(Result::unwrap)
            .max()
            .expect("No max found.")
    }).max().expect("No max found.")
}

fn main() {
    let default = "Failed to get panic message.".to_string();
    std::panic::set_hook(Box::new(move |p|{
        eprintln!("{}",p.payload().downcast_ref::<std::string::String>().unwrap_or(&default));
        std::process::exit(-1);
    }));
    let error = "CLI args broken.";
    let mut args = std::env::args();
    let arg_one = args.next().expect(error);
    let name = Path::new(arg_one.as_str()).file_name().and_then(OsStr::to_str).expect(error);
    let a: Vec<_> = std::env::args().skip(1).collect();
    let snark = "What did you call me?";
    let m:i64 = if a.is_empty() {
        let s = std::io::stdin();
        let m = match name {
            "min" => iter_to_min(s.lock().lines().map(Result::unwrap)),
            "max" => iter_to_max(s.lock().lines().map(Result::unwrap)),
            _ => panic!(snark)
        };
        m
    }else{
        match name {
            "min" => iter_to_min(a.into_iter()),
            "max" => iter_to_max(a.into_iter()),
            _ => panic!(snark)
        }
    };
    println!("{}", m);
}


#[cfg(test)]
mod tests {
    use super::iter_to_min;
    use super::iter_to_max;

    #[test]
    fn single_line() {
        let a = vec!["0 1 2".to_string()];
        let b = a.clone();
        assert_eq!(0, iter_to_min(a.into_iter()));
        assert_eq!(2, iter_to_max(b.into_iter()));
    }

    #[test]
    fn two_lines() {
        let a = vec!["3 4 5 6 7 ".to_string(), "0 1 2".to_string()];
        let b = a.clone();
        assert_eq!(0, iter_to_min(a.into_iter()));
        assert_eq!(7, iter_to_max(b.into_iter()));
    }

    #[test]
    fn negatives() {
        let a = vec!["-1".to_string(), " 3 4 5 6 7 ".to_string(), "0 1 2".to_string()];
        let b = a.clone();
        assert_eq!(-1, iter_to_min(a.into_iter()));
        assert_eq!(7, iter_to_max(b.into_iter()));
    }

    #[test]
    fn all_negatives() {
        let a = vec!["-1".to_string(), " -3 -4 -5 -6 -7 ".to_string(), "0 -1 -2".to_string()];
        let b = a.clone();
        assert_eq!(-7, iter_to_min(a.into_iter()));
        assert_eq!(0, iter_to_max(b.into_iter()));
    }
}
