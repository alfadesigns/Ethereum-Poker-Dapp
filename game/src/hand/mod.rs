#![allow(dead_code, unused_imports)]
mod test;

use card::{Card, Suit};
use rankings::{get_rank, Ranks, Rank};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Hand<'a> {
   pub cards: Vec<&'a Card>,
   value: Option<Rank<'a>>
}

impl<'a> Hand<'a> {
  pub fn new(cards: Vec<&'a Card>) -> Hand<'a> {
    Hand{cards, value: None}
  }

  pub fn get_value(&mut self) {
    if let Some(rank) = get_rank(&self) {
        self.value = Some(Rank{cards: vec![], rank}); 
    }
  }

  pub fn ranks(&mut self) -> Vec<u8> {
    self.cards.sort();
    let mut ranks: Vec<u8> = Vec::new();
    for card in self.cards.iter() {
        ranks.push(card.rank().first().unwrap().0);
    }
    ranks
  }

  pub fn suits(&self) -> HashMap<Suit, u8> {
      let mut suits: HashMap<Suit, u8>  = HashMap::new();
      for card in self.cards.iter() {
          let number = suits.entry(card.suit).or_insert(0);
          *number += 1;
      }
      suits
  }
}

