#[cfg(test)]
use crate::*;

#[test]
fn deck_shuffle() {
    assert_ne!(new_standard_deck(), new_standard_deck().shuffle());
    assert_ne!(new_italian_deck(), new_italian_deck().shuffle());
}

#[test]
fn shuffle_onehundred() {
    for _ in 0..100 {
        assert_ne!(new_standard_deck(), new_standard_deck().shuffle());
        assert_ne!(new_italian_deck(), new_italian_deck().shuffle());
    }
}

#[test]
fn create_new_standard_deck() {
    let manual_standard_deck = Deck {
        deck_type: DeckType::Standard,
        cards: vec![
            Card {
                suit: "clubs".to_string(),
                value: 1,
            },
            Card {
                suit: "clubs".to_string(),
                value: 2,
            },
            Card {
                suit: "clubs".to_string(),
                value: 3,
            },
            Card {
                suit: "clubs".to_string(),
                value: 4,
            },
            Card {
                suit: "clubs".to_string(),
                value: 5,
            },
            Card {
                suit: "clubs".to_string(),
                value: 6,
            },
            Card {
                suit: "clubs".to_string(),
                value: 7,
            },
            Card {
                suit: "clubs".to_string(),
                value: 8,
            },
            Card {
                suit: "clubs".to_string(),
                value: 9,
            },
            Card {
                suit: "clubs".to_string(),
                value: 10,
            },
            Card {
                suit: "clubs".to_string(),
                value: 11,
            },
            Card {
                suit: "clubs".to_string(),
                value: 12,
            },
            Card {
                suit: "clubs".to_string(),
                value: 13,
            },
            Card {
                suit: "diamonds".to_string(),
                value: 1,
            },
            Card {
                suit: "diamonds".to_string(),
                value: 2,
            },
            Card {
                suit: "diamonds".to_string(),
                value: 3,
            },
            Card {
                suit: "diamonds".to_string(),
                value: 4,
            },
            Card {
                suit: "diamonds".to_string(),
                value: 5,
            },
            Card {
                suit: "diamonds".to_string(),
                value: 6,
            },
            Card {
                suit: "diamonds".to_string(),
                value: 7,
            },
            Card {
                suit: "diamonds".to_string(),
                value: 8,
            },
            Card {
                suit: "diamonds".to_string(),
                value: 9,
            },
            Card {
                suit: "diamonds".to_string(),
                value: 10,
            },
            Card {
                suit: "diamonds".to_string(),
                value: 11,
            },
            Card {
                suit: "diamonds".to_string(),
                value: 12,
            },
            Card {
                suit: "diamonds".to_string(),
                value: 13,
            },
            Card {
                suit: "hearts".to_string(),
                value: 1,
            },
            Card {
                suit: "hearts".to_string(),
                value: 2,
            },
            Card {
                suit: "hearts".to_string(),
                value: 3,
            },
            Card {
                suit: "hearts".to_string(),
                value: 4,
            },
            Card {
                suit: "hearts".to_string(),
                value: 5,
            },
            Card {
                suit: "hearts".to_string(),
                value: 6,
            },
            Card {
                suit: "hearts".to_string(),
                value: 7,
            },
            Card {
                suit: "hearts".to_string(),
                value: 8,
            },
            Card {
                suit: "hearts".to_string(),
                value: 9,
            },
            Card {
                suit: "hearts".to_string(),
                value: 10,
            },
            Card {
                suit: "hearts".to_string(),
                value: 11,
            },
            Card {
                suit: "hearts".to_string(),
                value: 12,
            },
            Card {
                suit: "hearts".to_string(),
                value: 13,
            },
            Card {
                suit: "spades".to_string(),
                value: 1,
            },
            Card {
                suit: "spades".to_string(),
                value: 2,
            },
            Card {
                suit: "spades".to_string(),
                value: 3,
            },
            Card {
                suit: "spades".to_string(),
                value: 4,
            },
            Card {
                suit: "spades".to_string(),
                value: 5,
            },
            Card {
                suit: "spades".to_string(),
                value: 6,
            },
            Card {
                suit: "spades".to_string(),
                value: 7,
            },
            Card {
                suit: "spades".to_string(),
                value: 8,
            },
            Card {
                suit: "spades".to_string(),
                value: 9,
            },
            Card {
                suit: "spades".to_string(),
                value: 10,
            },
            Card {
                suit: "spades".to_string(),
                value: 11,
            },
            Card {
                suit: "spades".to_string(),
                value: 12,
            },
            Card {
                suit: "spades".to_string(),
                value: 13,
            },
        ],
    };
    assert_eq!(manual_standard_deck, new_standard_deck());
}

#[test]
fn create_new_italian_deck() {
    let manual_italian_deck = Deck {
        deck_type: DeckType::Italian,
        cards: vec![
            Card {
                suit: "spade".to_string(),
                value: 1,
            },
            Card {
                suit: "spade".to_string(),
                value: 2,
            },
            Card {
                suit: "spade".to_string(),
                value: 3,
            },
            Card {
                suit: "spade".to_string(),
                value: 4,
            },
            Card {
                suit: "spade".to_string(),
                value: 5,
            },
            Card {
                suit: "spade".to_string(),
                value: 6,
            },
            Card {
                suit: "spade".to_string(),
                value: 7,
            },
            Card {
                suit: "spade".to_string(),
                value: 8,
            },
            Card {
                suit: "spade".to_string(),
                value: 9,
            },
            Card {
                suit: "spade".to_string(),
                value: 10,
            },
            Card {
                suit: "coppe".to_string(),
                value: 1,
            },
            Card {
                suit: "coppe".to_string(),
                value: 2,
            },
            Card {
                suit: "coppe".to_string(),
                value: 3,
            },
            Card {
                suit: "coppe".to_string(),
                value: 4,
            },
            Card {
                suit: "coppe".to_string(),
                value: 5,
            },
            Card {
                suit: "coppe".to_string(),
                value: 6,
            },
            Card {
                suit: "coppe".to_string(),
                value: 7,
            },
            Card {
                suit: "coppe".to_string(),
                value: 8,
            },
            Card {
                suit: "coppe".to_string(),
                value: 9,
            },
            Card {
                suit: "coppe".to_string(),
                value: 10,
            },
            Card {
                suit: "denari".to_string(),
                value: 1,
            },
            Card {
                suit: "denari".to_string(),
                value: 2,
            },
            Card {
                suit: "denari".to_string(),
                value: 3,
            },
            Card {
                suit: "denari".to_string(),
                value: 4,
            },
            Card {
                suit: "denari".to_string(),
                value: 5,
            },
            Card {
                suit: "denari".to_string(),
                value: 6,
            },
            Card {
                suit: "denari".to_string(),
                value: 7,
            },
            Card {
                suit: "denari".to_string(),
                value: 8,
            },
            Card {
                suit: "denari".to_string(),
                value: 9,
            },
            Card {
                suit: "denari".to_string(),
                value: 10,
            },
            Card {
                suit: "bastoni".to_string(),
                value: 1,
            },
            Card {
                suit: "bastoni".to_string(),
                value: 2,
            },
            Card {
                suit: "bastoni".to_string(),
                value: 3,
            },
            Card {
                suit: "bastoni".to_string(),
                value: 4,
            },
            Card {
                suit: "bastoni".to_string(),
                value: 5,
            },
            Card {
                suit: "bastoni".to_string(),
                value: 6,
            },
            Card {
                suit: "bastoni".to_string(),
                value: 7,
            },
            Card {
                suit: "bastoni".to_string(),
                value: 8,
            },
            Card {
                suit: "bastoni".to_string(),
                value: 9,
            },
            Card {
                suit: "bastoni".to_string(),
                value: 10,
            },
        ],
    };
    assert_eq!(manual_italian_deck, new_italian_deck());
}
