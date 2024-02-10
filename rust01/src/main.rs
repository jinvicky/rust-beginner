use std::fs;
use text_colorizer::Colorize;

use crate::colorizer::args_colorizer;
use crate::colorizer::args_colorizer::replace;
use crate::http1::server;

mod http1 {
    pub mod server;
}

mod colorizer {
    pub mod args_colorizer;
}

/* TcpListener로 기본 get 요청과 404 처리해보기 */
// fn main() {
//     // server::run();
// }


/* 파일 내용 변경 및 복사본 만들기, 실패시 글자 색을 red로 해서 출력하기 */
fn main2() {

    let args = args_colorizer::parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(V) => V,
        Err(e) => {
            eprintln!("{}: failed to read file '{}': {:?}",
                      "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };


    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(V) => V,
        Err(e) => {
            eprintln!("{}: failed to replace text: {:?}"
                      , "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{}: failed to write file '{}': {:?}",
                      "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    }
    /*
    * parse_args() 함수에서 인수를 가져온 다음에, 파일 이름을 꺼내서 read_to_string(), write에 전달한다.
    * Copy.toml 파일을 생성한다.
    * 오류가 발생하면 색을 곁들여서 출력한다.
    */
}

fn main() {}
