use super::App;
use crate::algorithms;
use crate::data::{AnyEntry, EntryCore, IEntry, UEntry};
use crate::utils;
use crossterm::event;
use crossterm::style::Stylize;
use itertools::Itertools;

impl App {
    pub fn choose_indexed(&self, query: &str) -> Option<IEntry> {
        prompt_choose(
            query,
            self.ientries.as_ref(),
            self.config.sw_nw_ratio,
            self.config.search_threshold,
        )
    }
    pub fn choose_unindexed(&self, query: &str) -> Option<UEntry> {
        prompt_choose(
            query,
            self.uentries.as_ref(),
            self.config.sw_nw_ratio,
            self.config.search_threshold,
        )
    }
    pub fn choose_any<'a>(
        &'a self,
        query: &str,
        anyentries_opt: Option<&'a [AnyEntry]>,
    ) -> Option<AnyEntry<'a>> {
        if let Some(anyentries) = anyentries_opt {
            prompt_choose(
                query,
                anyentries,
                self.config.sw_nw_ratio,
                self.config.search_threshold,
            )
        } else {
            let anyentries = self.get_anyentries();
            prompt_choose(
                query,
                &anyentries,
                self.config.sw_nw_ratio,
                self.config.search_threshold,
            )
        }
    }
}

fn prompt_choose<T: EntryCore>(
    query: &str,
    entries: &[T],
    sw_nw_ratio: f64,
    search_threshold: f64,
) -> Option<T> {
    let ranked_choices: Vec<&T> = entries
        .iter()
        .map(|entry| {
            let score = std::iter::once(entry.get_name())
                .chain(entry.get_aliases().iter().map(String::as_str))
                .map(|string| {
                    sw_nw_ratio * algorithms::smith_waterman(string, query)
                        + (1.0 - sw_nw_ratio) * algorithms::needleman_wunsch(string, query)
                })
                .fold(-1.0, |acc, x| if x > acc { x } else { acc });
            (entry, score)
        })
        // Keeps entries with a high enough score
        .filter(|(_, score)| *score >= search_threshold)
        // Sorts by score, highest at the top
        .sorted_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap())
        // Keep only the top 3 scores
        .take(3)
        .map(|(entry, _)| entry)
        .collect();

    let response = match ranked_choices.len() {
        0 => None,
        1 => prompt_choose_single(ranked_choices[0]),
        2.. => prompt_choose_multiple(&ranked_choices),
    };

    if let Some(entry) = response.as_ref() {
        println!("{} {}", utils::ARROW.cyan().dim(), entry.get_name().dim())
    }
    println!();
    response
}

fn prompt_choose_single<T: EntryCore>(entry: &T) -> Option<T> {
    println!("{} [Y/n]", entry.print_name_and_first_alias());
    utils::enter_clean_input_mode();
    let option = loop {
        if !event::poll(std::time::Duration::from_millis(200)).unwrap_or_else(|err| {
            eprintln!("Warning: Event polling issue, '{}'", err);
            false
        }) {
            continue;
        }
        let key = match event::read() {
            Ok(event::Event::Key(key)) => key,
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Warning: Event read issue, '{}'", err);
                continue;
            }
        };

        if key.kind != event::KeyEventKind::Press {
            continue;
        }

        match key.code {
            event::KeyCode::Esc
            | event::KeyCode::Char('Q')
            | event::KeyCode::Char('q')
            | event::KeyCode::Char('N')
            | event::KeyCode::Char('n') => {
                break None;
            }
            event::KeyCode::Char('y') | event::KeyCode::Enter => {
                break Some(entry.clone());
            }
            _ => (),
        }
    };
    utils::exit_clean_input_mode();
    option
}

fn prompt_choose_multiple<T: EntryCore>(entries: &[&T]) -> Option<T> {
    for (idx, &entry) in entries.iter().enumerate() {
        println!("{}. {}", idx + 1, entry.print_name_and_first_alias());
    }
    utils::enter_clean_input_mode();
    let option = loop {
        if !event::poll(std::time::Duration::from_millis(200)).unwrap_or_else(|err| {
            eprintln!("Warning: Event polling issue: '{}'", err);
            false
        }) {
            continue;
        }
        let key = match event::read() {
            Ok(event::Event::Key(key)) => key,
            Ok(_) => continue,
            Err(err) => {
                eprintln!("Warning: Event read issue, '{}'", err);
                continue;
            }
        };

        if key.kind != event::KeyEventKind::Press {
            continue;
        }

        match key.code {
            event::KeyCode::Esc
            | event::KeyCode::Char('Q')
            | event::KeyCode::Char('q')
            | event::KeyCode::Char('N')
            | event::KeyCode::Char('n') => {
                break None;
            }
            event::KeyCode::Enter => {
                break Some(entries[0].clone());
            }
            event::KeyCode::Char(chr) => {
                let idx = match chr {
                    '1' => 0,
                    '2' => 1,
                    '3' => 2,
                    '4' => 3,
                    '5' => 4,
                    '6' => 5,
                    '7' => 6,
                    '8' => 7,
                    '9' => 8,
                    _ => continue,
                };
                if let Some(&entry) = entries.get(idx) {
                    break Some(entry.clone());
                }
            }
            _ => (),
        }
    };
    utils::exit_clean_input_mode();
    option
}
