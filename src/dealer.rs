use crate::{models::{player::{Player}, WithId}, card_manager::Card};

#[derive(Debug, Clone)]
pub struct Dealer {}

impl Dealer {
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