// This file is part of Solitude.
//     Solitude is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//     Solitude is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//     You should have received a copy of the GNU General Public License along with Solitude. If not, see <https://www.gnu.org/licenses/>.

use rand::Rng;

pub mod pyramid;

// Card struct:
//     Holds the number value of the card and, if it exists, the string representing its face value
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    pub value: u32,
    pub suit: Suit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    Spade,
    Coppe,
    Denari,
    Bastoni,
}

// Enum that specifies the kind of deck
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeckType {
    Standard,
    Italian,
}

pub fn new_standard_deck() -> Deck {
    new_deck(
        DeckType::Standard,
        13,
        vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades],
    )
}

pub fn new_italian_deck() -> Deck {
    new_deck(
        DeckType::Italian,
        10,
        vec![Suit::Spade, Suit::Coppe, Suit::Denari, Suit::Bastoni],
    )
}

// Create a specific deck
fn new_deck(deck_type: DeckType, per_suit: u32, suits: Vec<Suit>) -> Deck {
    let mut cards = Vec::new();

    for s in suits {
        for j in 1..=per_suit {
            cards.push(Card { value: j, suit: s });
        }
    }

    Deck { deck_type, cards }
}

// Deck struct:
//     Holds the cards of the deck in a particular order
#[derive(Debug, PartialEq)]
pub struct Deck {
    pub deck_type: DeckType,
    pub cards: Vec<Card>,
}

fn get_shuffle_order(remainder: &[u32]) -> Vec<u32> {
    let random_slice_index = rand::thread_rng().gen_range(0..remainder.len());

    if remainder.len() == 1 {
        return vec![remainder[0]];
    }
    if random_slice_index == 0 {
        return [
            &[remainder[random_slice_index]],
            &get_shuffle_order(&remainder[1..])[..],
        ]
        .concat();
    }
    if random_slice_index == remainder.len() {
        return [
            &[remainder[random_slice_index]],
            &get_shuffle_order(&remainder[0..random_slice_index])[..],
        ]
        .concat();
    }

    [
        &[remainder[random_slice_index]],
        &get_shuffle_order(
            &[
                &remainder[0..random_slice_index],
                &remainder[(random_slice_index + 1)..],
            ]
            .concat(),
        )[..],
    ]
    .concat()
}

impl Deck {
    pub fn shuffle(&self) -> Deck {
        let index_vec = (0..self.cards.len() as u32).collect::<Vec<u32>>();
        let shuffle_order = get_shuffle_order(&index_vec[..]);
        let mut cards = Vec::new();

        for i in shuffle_order {
            cards.push(self.cards[i as usize])
        }

        Deck {
            deck_type: self.deck_type,
            cards,
        }
    }
}

mod tests;
