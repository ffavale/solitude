// This file is part of Solitude.
//     Solitude is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//     Solitude is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//     You should have received a copy of the GNU General Public License along with Solitude. If not, see <https://www.gnu.org/licenses/>.

use rand::Rng;

pub mod pyramid;

// Card struct:
//     Holds the number value of the card and, if it exists, the string representing its face value
#[derive(Clone, PartialEq, Debug)]
pub struct Card {
    pub value: u32,
    pub suit: String,
}

// Enum that specifies the kind of deck
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DeckType {
    Standard,
    Italian,
}

pub fn new_standard_deck() -> Deck {
    new_deck(
        DeckType::Standard,
        13,
        vec!["clubs", "diamonds", "hearts", "spades"],
    )
}

pub fn new_italian_deck() -> Deck {
    new_deck(
        DeckType::Italian,
        10,
        vec!["spade", "coppe", "denari", "bastoni"],
    )
}

// Create a specific deck
fn new_deck(d_type: DeckType, per_suit: u32, suits: Vec<&str>) -> Deck {
    let mut cards = Vec::new();

    for (_, suit) in suits.iter().enumerate() {
        for j in 1..=per_suit {
            cards.push(Card {
                value: j,
                suit: suit.to_string(),
            })
        }
    }

    Deck {
        deck_type: d_type,
        cards,
    }
}

// Deck struct:
//     Holds the cards of the deck in a particular order
#[derive(PartialEq, Debug)]
pub struct Deck {
    pub deck_type: DeckType,
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn shuffle(&self) -> Deck {
        let deck_legnth = self.cards.len();
        let mut rng = rand::thread_rng();
        let mut current_order = self.cards.clone();
        let mut shufl_cards = Vec::new();

        for (i, _) in self.cards.iter().enumerate() {
            let gotten_index = rng.gen_range(0..(deck_legnth - i));
            shufl_cards.push(current_order[gotten_index].clone());
            current_order.remove(gotten_index);
        }

        Deck {
            deck_type: self.deck_type,
            cards: shufl_cards,
        }
    }
}

mod tests;
