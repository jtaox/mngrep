use std::fs;
use std::error::Error;
use std::env;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitvie(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

  contents.lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitvie<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut query_results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      query_results.push(line);
    }
  }

  query_results
}

pub struct Config {
  query: String,
  filename: String,
  case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

      args.next();

      let query = match args.next() {
        Some(arg) => arg,
        None => return Err("输入查询内容")
      };

      let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("输入查询的文件")
      };

      if args.len() < 3 {
          return Err("参数错误");
      }

      let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

      Ok(Config { query, filename, case_sensitive })
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
rust
safe, fast, productive.
    ";

    assert_eq!(vec![
      "safe, fast, productive."
    ], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "Duct";
    let contents = "\
rust
safe, fast, productive.
    ";

    assert_eq!(vec![
      "safe, fast, productive."
    ], search_case_insensitvie(query, contents));
  }
}