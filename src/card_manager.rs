use rand::thread_rng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

const ALL_POWERFUL_MAGIC_CARDS: &[CardValue] = &[CardValue::Two, CardValue::Three, CardValue::Seven];
const MAGIC_CARDS: &[CardValue] = &[CardValue::Ten, CardValue::Ace];

#[derive(Debug, EnumIter, PartialEq, Clone)]
enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds
}

impl Suit {
    fn icon(&self) -> char {
        match self {
            Suit::Diamonds => '♦',
            Suit::Clubs => '♣',
            Suit::Hearts => '♥',
            Suit::Spades => '♠',
        }
    }
}

#[derive(Debug, Display, EnumIter, PartialEq, Clone, Copy)]
pub enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl CardValue {
    pub fn str_num_value(&self) -> (String, i8) {
        match self {
            CardValue::Ace => (String::from("A"), 14),
            CardValue::Two => (String::from("2"), 2),
            CardValue::Three => (String::from("3"), 3),
            CardValue::Four => (String::from("4"), 4),
            CardValue::Five => (String::from("5"), 5),
            CardValue::Six => (String::from("6"), 6),
            CardValue::Seven => (String::from("7"), 7),
            CardValue::Eight => (String::from("8"), 8),
            CardValue::Nine => (String::from("9"), 9),
            CardValue::Ten => (String::from("10"), 10),
            CardValue::Jack => (String::from("J"), 11),
            CardValue::Queen => (String::from("Q"), 12),
            CardValue::King => (String::from("K"), 13),
        }
    }

    pub fn parse(value_str: Option<String>) -> Option<CardValue> {
        value_str.and_then(|s| match s.as_str() {
            "2" => Some(CardValue::Two),
            "3" => Some(CardValue::Three),
            "4" => Some(CardValue::Four),
            "5" => Some(CardValue::Five),
            "6" => Some(CardValue::Six),
            "7" => Some(CardValue::Seven),
            "8" => Some(CardValue::Eight),
            "9" => Some(CardValue::Nine),
            "10" => Some(CardValue::Ten),
            "J" => Some(CardValue::Jack),
            "Q" => Some(CardValue::Queen),
            "K" => Some(CardValue::King),
            "A" => Some(CardValue::Ace),
            _ => None,
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Card {
    pub value: CardValue,
    suit: Suit,
    pub short_name: String,
    pub num_value: i8,
}

impl Card {
    pub fn parse_card(input: &str) -> Option<Card> {
        let mut chars = input.chars();
    
        let value = match chars.next()? {
            'A' => CardValue::Ace,
            '2' => CardValue::Two,
            '3' => CardValue::Three,
            '4' => CardValue::Four,
            '5' => CardValue::Five,
            '6' => CardValue::Six,
            '7' => CardValue::Seven,
            '8' => CardValue::Eight,
            '9' => CardValue::Nine,
            '1' => {
                if chars.next() != Some('0') {
                    return None;
                }
                CardValue::Ten
            },
            'J' => CardValue::Jack,
            'Q' => CardValue::Queen,
            'K' => CardValue::King,
            _ => return None,
        };
    
        let suit = match chars.next()? {
            '♠' => Suit::Spades,
            '♣' => Suit::Clubs,
            '♥' => Suit::Hearts,
            '♦' => Suit::Diamonds,
            _ => return None,
        };
    
        let (_, num_value) = value.str_num_value();
    
        Some(Card {
            value,
            short_name: input.to_string(),
            num_value,
            suit
        })
    }

    pub fn can_be_played_on(&self, current_value: &Option<CardValue>) -> bool {
        if ALL_POWERFUL_MAGIC_CARDS.contains(&self.value) {
            return true
        }
        if MAGIC_CARDS.contains(&self.value) && *current_value != Some(CardValue::Seven) {
            return true
        }
        match current_value {
            Some(CardValue::Two) => true,
            Some(CardValue::Three) => {
                panic!("you can not have an invisible as a value");
            },
            Some(CardValue::Four) => true,
            Some(CardValue::Five) => self.num_value >= 5,
            Some(CardValue::Six) => self.num_value >= 6,
            Some(CardValue::Seven) => self.num_value <= 7,
            Some(CardValue::Eight) => self.num_value >= 8,
            Some(CardValue::Nine) => self.num_value >= 9,
            Some(CardValue::Ten) => self.num_value >= 10,
            Some(CardValue::Jack) => self.num_value >= 11,
            Some(CardValue::Queen) => self.num_value >= 12,
            Some(CardValue::King) => self.num_value >= 13,
            Some(CardValue::Ace) => false,
            None => true,
        }
    }
}

#[derive(Debug)]
pub struct CardManager {}

impl CardManager {
    pub fn new() -> Self {
        CardManager{}
    }

    pub fn new_deck(&self, num_decks: i8) -> Vec<String> {
        let mut deck: Vec<String> = vec![];
        for _ in 0..num_decks {
            for suit in Suit::iter() {
                for value in CardValue::iter() {
                    let suit = suit.clone();
                    deck.push(format!("{}{}", value.str_num_value().0, suit.icon()));
                }
            }
        }
        
        deck.shuffle(&mut thread_rng());
        deck
    }
}
