use regex::Regex;
use std::env;
use std::process;

mod file_search;

fn main() {
    let args: Vec<String> = env::args().collect(); 

    // 参数1：搜索目录； 参数2：要搜索的正则表达式。
    if args.len() < 3 {
        eprintln!("使用方式：{} <目标目录> <要搜索的正则表达式>", args[0]);
        process::exit(1);
    }

    let verbose = args.contains(&String::from("-v")) || args.contains(&String::from("--verbose"));
    
    let pattern = &args[2];
    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("无效的正则表达式 '{}': {}", pattern, err);
            process::exit(1);
        }
    };

    match file_search::find(&args[1], &regex, verbose) {
        Ok(matches) => {
            if matches.is_empty() {
                println!("未找到匹配项。");
            } else {
                println!("找到以下匹配项：");
                for file in matches {
                    println!("{}", file);
                }
            }
        }
        Err(error) => {
            eprintln!("发生错误: {}", error);
            process::exit(1);
        }
    }
}
