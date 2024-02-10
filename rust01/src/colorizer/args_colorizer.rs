use text_colorizer::*;
use regex::Regex;

#[derive(Debug)]
pub struct Arguments {
    pub(crate) target: String,
    pub(crate) replacement: String,
    pub(crate) filename: String,
    pub(crate) output: String,
}

pub fn print_usage() {
    eprintln!("{} - change occurrences of a string into another ",
              "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

pub fn parse_args() -> Arguments {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{}: wrong number of arguments; expected 4 got {}.", "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }

}

/* 찾아 바꾸기 함수, 정규 표현식과 매칭되는 부분을 찾아서 주어진 대체 문자열로 바꾼다.  */
pub fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}