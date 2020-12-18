use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

pub type Solution = ((String, String), Duration);

pub fn day_input_filename(day: u8) -> PathBuf {
    let padded_day = format!("{:02}", day);
    Path::new("inputs").join(format!("day{}.in", padded_day))
}

pub fn get_input(path: &Path) -> Result<Vec<String>, std::io::Error> {
    Ok(fs::read_to_string(path)
        .expect(&format!("Input file not found: {:?}", path))
        .lines()
        .map(&str::to_string)
        .collect())
}

pub fn get_day_input(day: u8) -> Result<Vec<String>, std::io::Error> {
    get_input(&day_input_filename(day))
}

#[macro_export]
macro_rules! reparse_one {
    ($err:ident, $res:expr , $($arg1:tt)::* ) => {{
        let err = "0".parse::<$($arg1)::*>().unwrap();
        match $res.next() {
            Some(item) => {
                let ret = item.unwrap().as_str().parse::<$($arg1)::*>();
                if ret.is_err() {
                    println!("err in parsing {:?}", item);
                    $err = concat!("parse::", stringify!($($arg1)::*));
                }
                ret.unwrap_or(err)
            }
            _ => {
                $err = concat!("internal ", stringify!($($arg1)::*));
                err
            }
        }
    }};
}

#[macro_export]
macro_rules! reparse {
    ($txt:expr, $re:expr, $($($args:tt)::*),* ) => {
        {
            let mut err = "" ;
            if let Some(captures) = $re.captures($txt) {
                let mut matches = captures.iter();
                matches.next();
                let result = ($($crate::reparse_one!(err,matches,$($args)::*)),*) ;
                if err == "" {
                    Ok(result)
                } else {
                    Err(err)
                }
            } else {
                Err(err)
            }
        }
    };
    ($txt:expr, compile $re:expr, $($($args:tt)::*),* ) => {
        {
            let mut err = "" ;
            let matched = Regex::new($re).unwrap().captures($txt).unwrap();
            let mut matches = matched.iter();
            matches.next();
            let result = ($($crate::reparse_one!(err,matches,$($args)::*)),*) ;
            if err == "" {
                Ok(result)
            } else {
                Err(err)
            }
        }
    };
}

#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         [$(($key, $val)),*].iter().cloned().collect()
    }}
}
