use std::io::{BufferedReader, EndOfFile, File, IoError};

#[deriving(Eq, PartialEq)]
enum CardSuit {
    Hearts,
    Clubs,
    Spades,
    Diamonds,
}

#[deriving(Eq, FromPrimitive, Ord, PartialEq, PartialOrd)]
enum CardRank {
     Two = 0,
     Three = 1,
     Four = 2,
     Five = 3,
     Six = 4,
     Seven = 5,
     Eight = 6,
     Nine = 7,
     Ten = 8,
     Jack = 9,
     Queen = 10,
     King = 11,
     Ace = 12,
}

struct Card {
    rank: CardRank,
    suit: CardSuit,
}

#[deriving(Eq, PartialEq)]
enum HandRank {
    HighCard(CardRank),
    OnePair(CardRank),
    TwoPairs(CardRank, CardRank),
    ThreeOfAKind(CardRank),
    Straight(CardRank),
    Flush(CardSuit),
    FullHouse(CardRank, CardRank),
    FourOfAKind(CardRank),
    StraightFlush(CardRank, CardSuit),
    RoyalFlush(CardSuit),
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &HandRank) -> Option<Ordering> {
        match (*self, *other) {
            (HighCard(a), HighCard(ref b)) => if a == *b { None } else { a.partial_cmp(b) },
            (HighCard(_), _) => Some(Less),
            (_, HighCard(_)) => Some(Greater),
            (OnePair(a), OnePair(ref b)) => if a == *b { None } else { a.partial_cmp(b) },
            (OnePair(_), _) => Some(Less),
            (_, OnePair(_)) => Some(Greater),
            (TwoPairs(a, c), TwoPairs(ref b, ref d)) =>
                if a == *b { if c == *d { None } else { c.partial_cmp(d) } } else { a.partial_cmp(b) },
            (TwoPairs(..), _) => Some(Less),
            (_, TwoPairs(..)) => Some(Greater),
            (ThreeOfAKind(a), ThreeOfAKind(ref b)) => if a == *b { None } else { a.partial_cmp(b) },
            (ThreeOfAKind(_), _) => Some(Less),
            (_, ThreeOfAKind(_)) => Some(Greater),
            (Straight(a), Straight(ref b)) => if a == *b { None } else { a.partial_cmp(b) },
            (Straight(_), _) => Some(Less),
            (_, Straight(_)) => Some(Greater),
            (Flush(_), Flush(_)) => None,
            (Flush(_), _) => Some(Less),
            (_, Flush(_)) => Some(Greater),
            (FullHouse(a, c), FullHouse(ref b, ref d)) =>
                if a == *b { if c == *d { None } else { c.partial_cmp(d) } } else { a.partial_cmp(b) },
            (FullHouse(..), _) => Some(Less),
            (_, FullHouse(..)) => Some(Greater),
            (FourOfAKind(a), FourOfAKind(ref b)) => if a == *b { None } else { a.partial_cmp(b) },
            (FourOfAKind(_), _) => Some(Less),
            (_, FourOfAKind(_)) => Some(Greater),
            (StraightFlush(a, _), StraightFlush(ref b, _)) => if a == *b { None } else { a.partial_cmp(b) },
            (StraightFlush(..), _) => Some(Less),
            (_, StraightFlush(..)) => Some(Greater),
            (RoyalFlush(_), RoyalFlush(_)) => None,
        }
    }
}

struct Hand {
    cards: [Card, .. 5],
}

impl Hand {
    fn rank(&mut self) -> HandRank {
        // Start by putting the cards in order
        let ref mut cards = self.cards;
        cards.sort_by( |c1, c2| c1.rank.cmp(&c2.rank) );
        let mut top_card = unsafe { cards.unsafe_get(0) };
        let mut is_flush = true;
        let mut is_straight = true;
        let mut streak1: Option<(uint, CardRank)> = None;
        let mut streak2 = None;
        let mut consecutive = 1;
        for card in cards.iter().skip(1) {
            if card.suit != top_card.suit { is_flush = false; }
            if card.rank == top_card.rank {
                consecutive += 1;
                is_straight = false
            } else {
                if consecutive > 1 {
                    match (streak1, streak2) {
                        (None, _) => streak1 = Some((consecutive, top_card.rank)),
                        (Some(_), _) => streak2 = Some((consecutive, top_card.rank)),
                    }
                }
                consecutive = 1;
                if card.rank as uint != top_card.rank as uint + 1 { is_straight = false }
            }
            top_card = card;
        }
        if consecutive > 1 {
            match (streak1, streak2) {
                (None, _) => streak1 = Some((consecutive, top_card.rank)),
                (Some(_), _) => streak2 = Some((consecutive, top_card.rank)),
            }
        }
        match (is_flush, streak1, streak2) {
            (_, Some((4, rank)), None) => FourOfAKind(rank),
            (_, Some((3, rank1)), Some((2, rank2))) | (_, Some((2, rank2)), Some((3, rank1))) =>
                FullHouse(rank1, rank2),
            (true, Some(_), _) => Flush(top_card.suit),
            (true, None, _) => {
                if is_straight {
                    if top_card.rank == Ace {
                        RoyalFlush(top_card.suit)
                    } else {
                        StraightFlush(top_card.rank, top_card.suit)
                    }
                } else {
                    Flush(top_card.suit)
                }
            }
            (_, Some((3, rank)), _) => ThreeOfAKind(rank),
            (_, Some((2, rank1)), Some((2, rank2))) => TwoPairs(rank1, rank2),
            (_, Some((2, rank)), _) => OnePair(rank),
            _ => {
                if is_straight {
                    Straight(top_card.rank)
                } else {
                    HighCard(top_card.rank)
                }
            }
        }
    }
}

euler_problem!(b"142949df56ea8ae0be8b5306971900a4", w, {
    static path: &'static str = "data/poker.txt";
    let file = File::open(&Path::new(path)); 
    let mut reader = BufferedReader::new(file);
    let mut handbuf = [0u8, .. 30];
    let mut player1_wins = 0u16;
    loop {
        match reader.read_at_least(30, handbuf) {
            Err(IoError { kind: EndOfFile, .. }) => break,
            Ok(_) => (),
            Err(e) => return Err(e)
        };
        let mut player1: [Card, .. 5] = [unsafe { ::std::mem::uninitialized() }, .. 5];
        let mut player2: [Card, .. 5] = [unsafe { ::std::mem::uninitialized() }, .. 5];
        for (card, data) in player1.mut_iter().chain(player2.mut_iter()).zip(handbuf.chunks(3)) {
            let rank = match data[0] {
                b'2' => Two,
                b'3' => Three,
                b'4' => Four,
                b'5' => Five,
                b'6' => Six,
                b'7' => Seven,
                b'8' => Eight,
                b'9' => Nine,
                b'T' => Ten,
                b'J' => Jack,
                b'Q' => Queen,
                b'K' => King,
                b'A' => Ace,
                _ => return Err(IoError::last_error())
            };
            let suit = match data[1] {
                b'S' => Spades,
                b'C' => Clubs,
                b'H' => Hearts,
                b'D' => Diamonds,
                _ => return Err(IoError::last_error())
            };
            *card = Card { rank: rank, suit: suit };
        }
        let mut hand1 = Hand { cards: player1 };
        let mut hand2 = Hand { cards: player2 };
        let rank1 = hand1.rank();
        let rank2 = hand2.rank();
        match rank1.partial_cmp(&rank2) {
            Some(Less) => (),
            Some(Greater) => player1_wins += 1,
            Some(Equal) | None => {
                // Sort
                for (c1, c2) in hand1.cards.iter().rev().zip(hand2.cards.iter().rev()) {
                    match c1.rank.cmp(&c2.rank) {
                        Less => break,
                        Greater => { player1_wins += 1; break },
                        _ => ()
                    }
                }
            }
        }
    }
    write!(w, "{}", player1_wins)
})
