use std::collections::HashMap;
use std::error::Error;

use colored::*;
use indicatif::ProgressBar;
use libknox::*;
use log::*;
use sha1::{Digest, Sha1};

use crate::util::vault_path;

#[derive(PartialEq, Debug)]
pub(crate) enum PwnedResult {
  Clear,
  Pwned,
  Error(String),
}

pub(crate) fn pwned(args: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
  if args.is_present("path") {
    check_entry(args)
  } else {
    check_vault(args)
  }
}

fn check_entry(args: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
  let vault = VaultContext::open(vault_path()?)?;
  let path = args.value_of("path").unwrap();
  let entry = vault.read_entry(path)?;
  let pwnage = check_attributes(&entry.attributes);

  info!("Pwnage status for attributes at {}", path.bold());

  for (name, attribute) in pwnage {
    match attribute {
      PwnedResult::Error(err) => println!(
        " {} {} {}:{} ({})",
        "::".magenta().bold(),
        "ERROR".magenta(),
        path.bold(),
        name.dimmed(),
        err
      ),

      PwnedResult::Clear => println!(
        " {} {} {}:{}",
        "::".green().bold(),
        "CLEAR".green(),
        path.bold(),
        name.dimmed()
      ),

      PwnedResult::Pwned => println!(
        " {} {} {}:{}",
        "::".red().bold(),
        "PWNED".red(),
        path.bold(),
        name.dimmed()
      ),
    }
  }

  Ok(())
}

fn check_vault(_args: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
  let context = VaultContext::open(vault_path()?)?;
  let progress = ProgressBar::new(context.vault.get_index().len() as u64);
  let mut count = 0;

  info!("checking for pwned secret across your vault");

  for path in context.vault.get_index().keys() {
    let entry = context.read_entry(&path)?;

    for (name, attribute) in entry.attributes {
      if attribute.confidential && !attribute.file {
        if let AttributeValue::String(value) = attribute.value() {
          match check(&value) {
            PwnedResult::Pwned => {
              count += 1;

              progress.println(format!(
                " {} {} {}:{}",
                "::".red().bold(),
                "PWNED".red(),
                path.bold(),
                name.dimmed()
              ));
            }

            PwnedResult::Error(err) => {
              progress.println(format!(
                " {} {} {}:{} ({})",
                "::".red().bold(),
                "ERROR".red(),
                path.bold(),
                name.dimmed(),
                err
              ));
            }

            PwnedResult::Clear => (),
          }
        }
      }
    }

    progress.inc(1);
  }

  progress.finish();

  info!("{} secrets were found in HIBP's database", count);

  Ok(())
}

pub(crate) fn check_attributes(
  attributes: &HashMap<String, Attribute>,
) -> Vec<(String, PwnedResult)> {
  attributes
    .iter()
    .filter(|(_, attribute)| attribute.confidential && !attribute.file)
    .flat_map(|(name, attribute)| match attribute.value() {
      AttributeValue::String(value) => Some((name.to_string(), check(&value))),
      _ => None,
    })
    .collect()
}

fn check(value: &str) -> PwnedResult {
  let mut hasher = Sha1::default();
  hasher.input(value);

  let hash = format!("{:x}", hasher.result());
  let prefix = &hash[..5];

  let client = reqwest::Client::new()
    .get(&format!("https://api.pwnedpasswords.com/range/{}", prefix))
    .header("User-Agent", "knox (https://github.com/apognu/knox)")
    .send();

  match client {
    Err(err) => return PwnedResult::Error(err.description().to_string()),
    Ok(mut response) => {
      if response.status() != 200 {
        return PwnedResult::Error(format!("response code was {}", response.status()));
      }

      match response.text() {
        Err(err) => return PwnedResult::Error(err.description().to_string()),
        Ok(body) => {
          for line in body.lines() {
            let tokens: Vec<&str> = line.split(':').collect();
            if tokens.len() != 2 {
              continue;
            }

            if hash == format!("{}{}", prefix, tokens[0]).to_lowercase() {
              return PwnedResult::Pwned;
            }
          }
        }
      }
    }
  }

  PwnedResult::Clear
}

#[cfg(test)]
mod tests {
  use super::PwnedResult;
  use rand::{distributions::Alphanumeric, Rng};

  #[test]
  fn check() {
    assert_eq!(super::check("azerty"), PwnedResult::Pwned);

    let secure = rand::thread_rng()
      .sample_iter(&Alphanumeric)
      .take(64)
      .collect::<String>();

    // Dangerous, this could fail CI if random string is actually pwned
    assert_eq!(super::check(&secure), PwnedResult::Clear);
  }
}
