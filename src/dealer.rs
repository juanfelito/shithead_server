use crate::{models::{player::{Player}, WithId, discard::Discard, game::Game}, card_manager::{Card, CardValue}};

#[derive(Debug, Clone)]
pub struct Dealer {}

impl Dealer {
    pub fn successful_play<'a>(
        discard: &'a mut Discard,
        successful_cards: &'a mut Vec<String>
    ) {
        let discard_value = CardValue::parse(discard.current_value.clone());
        let played_value = Card::parse_card(&successful_cards[0]).expect("there's always a card").value;

        if discard_value == Some(played_value) {
            discard.repeat_count += successful_cards.len() as u32;
        } else {
            discard.repeat_count = successful_cards.len() as u32;
        }

        if played_value != CardValue::Three {
            discard.current_value = Some(played_value.str_num_value().0);
        }

        discard.cards.append(successful_cards);
        discard.current_card = discard.cards.last().cloned();
    }

    pub fn unsuccessful_play<'a>(
        player: &'a mut Player,
        discard: &'a mut Discard,
        mut unsuccessful_cards: &'a mut Vec<String>
    ) -> Vec<String> {
        let picked_cards = discard.cards.clone();
        player.cards.hand.append(&mut unsuccessful_cards);
        player.cards.hand.append(&mut discard.cards);

        Self::reset_discard_counters(discard);
        picked_cards
    }

    pub fn burn_discard<'a>(discard: &'a mut Discard) {
        discard.cards = vec![];
        Self::reset_discard_counters(discard);
    }

    fn reset_discard_counters<'a>(discard: &'a mut Discard) {
        discard.current_card = None;
        discard.current_value = None;
        discard.repeat_count = 0;
    }

    pub fn take_from_deck<'a>(player: &'a mut Player, deck: &'a mut Vec<String>) -> Vec<String> {
        let mut response = vec![];
        if deck.len() > 0 {
            let player_hand = &mut player.cards.hand;
            while player_hand.len() < 3 && deck.len() > 0 {
                let card = deck.pop().unwrap();
                response.push(card.clone());
                player_hand.push(card);
            }
        }

        response
    }

    pub fn tick_turn<'a>(game: &'a mut Game) {
        if game.turn < (game.users.as_ref().unwrap().len() - 1) as u32 {
            game.turn += 1;
        } else {
            game.turn = 0;
        }

        if game.players_out.contains(&game.turn) {
            Self::tick_turn(game)
        }
    }

    pub fn is_empty<'a>(player: &'a Player) -> bool {
        let cards = &player.cards;
        cards.hand.is_empty() && cards.face_up.is_empty() && cards.face_down.is_empty()
    }

    pub fn get_active_cards<'a>(player: &'a mut Player) -> &'a mut Vec<String> {
        match (player.cards.hand.len() > 0, player.cards.face_up.len() > 0) {
            (true, _) => return &mut player.cards.hand,
            (_, true) => return &mut player.cards.face_up,
            _ => return &mut player.cards.face_down,
        }
    }

    pub fn of_the_same_value(cards: &Vec<Card>) -> bool {
        if let Some(first_value) = cards.get(0).map(|c| c.value) {
            if !cards.iter().map(|c| c.value).all(|v| v == first_value) {
                return false
            }
        }

        true
    }

    pub fn initial_deal(deck: &mut Vec<String>, players: &mut Vec<WithId<Player>>) {
        Self::put_in_bucket(deck, players, CardBucket::FaceDown);
        Self::put_in_bucket(deck, players, CardBucket::FaceUp);
        Self::put_in_bucket(deck, players, CardBucket::Hand);
    }

    fn put_in_bucket(deck: &mut Vec<String>, players: &mut Vec<WithId<Player>>, bucket: CardBucket) {
        for _ in 0..3 {
            for (_, player) in players.iter_mut().enumerate() {
                if let Some(card) = deck.pop() {
                    match bucket {
                        CardBucket::Hand => player.inner.cards.hand.push(card),
                        CardBucket::FaceUp => player.inner.cards.face_up.push(card),
                        CardBucket::FaceDown => player.inner.cards.face_down.push(card),
                    }
                }
            }
        }
    }
}

enum CardBucket {
    Hand,
    FaceUp,
    FaceDown
}

#[cfg(test)]
mod tests {
    use surrealdb::sql::Thing;

    use super::*;

    #[test]
    fn test_put_in_bucket() {
        let mut cards = vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string(), "8".to_string()];
        let mut players = vec![WithId::new(Thing::from(("".to_string(), "".to_string())), Player::default()), WithId::new(Thing::from(("".to_string(), "".to_string())), Player::default())];

        Dealer::put_in_bucket(&mut cards, &mut players, CardBucket::FaceDown);

        assert_eq!(players[0].inner.cards.face_down.len(), 3);
        assert_eq!(players[0].inner.cards.face_down, vec!["8".to_string(), "6".to_string(), "4".to_string()]);
        assert_eq!(players[1].inner.cards.face_down.len(), 3);
        assert_eq!(players[1].inner.cards.face_down, vec!["7".to_string(), "5".to_string(), "3".to_string()]);
    }
}