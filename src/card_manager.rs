use rand::thread_rng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

#[derive(Debug, EnumIter, PartialEq, Clone)]
enum CardValue {
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
    fn str_num_value(&self) -> (String, i8) {
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
}

#[derive(PartialEq, Clone, Debug)]
struct Card {
    pub value: CardValue,
    pub suit: Suit,
    pub short_name: String,
    pub num_value: i8,
}

impl Card {
    fn new(value: CardValue, suit: Suit) -> Self {
        let (str_value, num_value) = value.str_num_value();
        Card {
            short_name: String::from(str_value) + &String::from(suit.icon()),
            num_value: num_value,
            value: value,
            suit: suit,
        }
    }

    fn can_be_played_on(&self, current_value: &Option<CardValue>) -> bool {
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
