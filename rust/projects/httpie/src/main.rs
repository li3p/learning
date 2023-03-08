use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use reqwest::Url;

#[derive(Parser, Debug)]
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand, Debug)]
enum Command {
    #[command(arg_required_else_help = true)]
    Get(Get),
    #[command(arg_required_else_help = true)]
    Post(Post),
}
#[derive(Debug, Args)]
struct Get {
    #[arg(value_parser=reqwest::Url::parse)]
    url: Url,
}
#[derive(Debug, Args)]
struct Post {
    #[arg(value_parser=reqwest::Url::parse)]
    url: Url,
    #[arg(value_parser=parse_kv_pair)]
    body: Vec<KvPair>,
}

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, Clone)]
struct KvPair {
    k: String,
    v: String,
}
/// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行 split，这会得到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
            // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为 value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}
/// 因为我们为 KvPair 实现了 FromStr，这里可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn main() {
    let opts = Cli::parse();
    println!("Hello, world! {:?}", opts.command);

    match opts.command {
        Command::Get(args) => println!("{:?}", args.url.to_string()),
        Command::Post(args) => println!("{:?}", args.url.to_string()),
    }
}
