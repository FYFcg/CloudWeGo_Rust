use regex::Regex;
use std::env;
use std::process;
use colored::*;

mod file_search;

fn main() {
    let args: Vec<String> = env::args().collect(); 

    // 参数1：搜索目录； 参数2：要搜索的正则表达式。
    if args.len() < 3 {
        eprintln!("{}{}{}", "使用方式：".red(), args[0].red(), "<目标目录> <要搜索的正则表达式> [-v|--verbose]".red());
        process::exit(1);
    }

    let verbose = args.contains(&String::from("-v")) || args.contains(&String::from("--verbose"));

    let len = args.len();
    let dir_begin = 1;
    let mut reg_begin = dir_begin + 1;

    while args[reg_begin].contains("/") {
        reg_begin += 1;
    }

    let end = if verbose { len - 1 } else { len };

    let mut sorted_matches: Vec<String> = Vec::new();

    for i in dir_begin..reg_begin {
        let mut unsorted_matches: Vec<String> = Vec::new();
        for j in reg_begin..end {
            let regex = match Regex::new(&args[j]) {
                Ok(re) => re,
                Err(err) => {
                    eprintln!("{} '{}': {}", "无效的正则表达式".red(),  &args[j].red(), err);
                    process::exit(1);
                }
            };
            match file_search::find(&args[i], &regex, verbose) {
                Ok(matches) => {
                    if matches.is_empty() {
                        unsorted_matches.clear();
                        break;
                    } else {
                        if j == reg_begin { 
                            unsorted_matches = matches;
                        } else {
                            let tmp_matches: Vec<&String> = matches.iter().filter(|&s| unsorted_matches.contains(s)).collect();
                            unsorted_matches.clear();
                            for s in tmp_matches {
                                unsorted_matches.push(s.to_string());
                            }
                        }
                    }
                }
                Err(error) => {
                    eprintln!("{}:{}", "发生错误".red(), error);
                    process::exit(1);
                }
            }
        }
        let filtered_matches: Vec<&String> = unsorted_matches.iter().filter(|&s| !sorted_matches.contains(s)).collect();
        for file in filtered_matches {
            sorted_matches.push(file.to_string());
        }
    }

    if sorted_matches.is_empty() {
        println!("{}", "未找到匹配项。".red());
    } else {
        println!("{}", "找到以下匹配项：".red());
        for file in sorted_matches {
            println!("{}", file.green());
        }
    }
}
