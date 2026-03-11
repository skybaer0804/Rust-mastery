// Minigrep — 간단한 CLI grep 도구

use std::env;
use std::process;

fn main() {
    // 커맨드라인 인자 읽기
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("인자 파싱 에러: {}", err); // stderr로 출력
        process::exit(1);
    });

    println!("검색어: '{}'", config.query);
    println!("대상 텍스트에서 검색합니다.\n");

    // 파일 대신 내장 텍스트 사용 (독립 실행을 위해)
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me, Rust is great.
RUST is also uppercase.";

    // 대소문자 구분 검색
    println!("=== 대소문자 구분 검색 ===");
    let results = search(&config.query, contents);
    for line in &results {
        println!("{}", line);
    }

    // 대소문자 무시 검색
    println!("\n=== 대소문자 무시 검색 ===");
    let results = search_case_insensitive(&config.query, contents);
    for line in &results {
        println!("{}", line);
    }
}

struct Config {
    query: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("검색어를 입력해주세요. 사용법: minigrep <검색어>");
        }

        let query = args[1].clone();
        Ok(Config { query })
    }
}

// 대소문자 구분 검색
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// 대소문자 무시 검색
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
