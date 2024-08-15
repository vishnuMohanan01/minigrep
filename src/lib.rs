use std::fs;
use std::error::Error;

pub struct Config {
  pub query: String,
  pub filename: String
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
      if args.len() < 3 {
          return Err("Not enough args!");
      }

      let query = args[1].clone();
      let filename = args[2].clone();

      Ok(Config { query, filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  for line in search(&contents, &config.query) {
    println!("{}", line);
  }

  Ok(())
}

fn search<'a>(content: &'a str, query: &'a str) -> Vec<&'a str> {
  let mut results= Vec::new();

  for line in content.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn search_returns_one_result() {
    let query = "duct";
    let content = "\
Rust:
Safe, fast, productive.
Pick three.
";

    assert_eq!(vec!["Safe, fast, productive."], search(content, query));
  }
}
