extern crate dotenv;

use dotenv::dotenv;
use std::env;


extern crate rand;

mod card;
mod player;
mod deck;
mod table;
mod hand;
mod rankings;
mod game_types;

use card::{Card, create as create_cards, allocate};
use deck::{Deck};
use card::{suit::Suit};
use rand::{thread_rng, Rng};
use std::{thread, time};

pub fn main() {
    let cards: Vec<Card> = create_cards();
    let deck = allocate(&cards);
	thread::spawn(|| {
		for i in 1..10 {
			let deck: Vec<Card> = Deck::new();
		}
	});

	let ten_millis = time::Duration::from_millis(10);
	let now = time::Instant::now();

	thread::sleep(ten_millis);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
