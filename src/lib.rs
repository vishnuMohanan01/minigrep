use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_insensitive: bool
}

fn string_to_bool(str: &str) -> bool {
  match str {
    "false" => false,
    "true" => true,
    _ => true
  }
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
      if args.len() < 3 {
          return Err("Not enough args!");
      }

      let query = args[1].clone();
      let filename = args[2].clone();

      let case_insensitive = env::var("CASE_INSENSITIVE").unwrap_or_else(|_| {"false".to_string()});
      let case_insensitive = string_to_bool(&case_insensitive);

      Ok(Config { query, filename, case_insensitive })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results: Vec<&str> = if config.case_insensitive {
    insensitive_search(&contents, &config.query)
  } else {
    search(&contents, &config.query)
  };

  for line in results {
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

fn insensitive_search<'a>(content: &'a str, query: &'a str) -> Vec<&'a str> {
  let mut results= Vec::new();

  for line in content.lines() {
    if line.to_lowercase().contains(&query.to_lowercase()) {
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

  #[test]
  fn insensitive_search_returns_case_insensitive_results() {
    let query = "Duct";
    let content = "\
Rust:
Safe, fast, productive.
Pick three.
";

    assert_eq!(vec!["Safe, fast, productive."], insensitive_search(content, query));
  }
}
