#![cfg(test)]

use crate::open_first;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardTypeStandard {
    _A,
    _K,
    _Q,
    _J,
    _T,
    _9,
    _8,
    _7,
    _6,
    _5,
    _4,
    _3,
    _2,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardTypeJokers {
    _A,
    _K,
    _Q,
    _T,
    _9,
    _8,
    _7,
    _6,
    _5,
    _4,
    _3,
    _2,
    _J,
}

impl CardTypeStandard {
    fn parse(c: char) -> Option<Self> {
        Some(match c {
            'A' => Self::_A,
            'K' => Self::_K,
            'Q' => Self::_Q,
            'J' => Self::_J,
            'T' => Self::_T,
            '9' => Self::_9,
            '8' => Self::_8,
            '7' => Self::_7,
            '6' => Self::_6,
            '5' => Self::_5,
            '4' => Self::_4,
            '3' => Self::_3,
            '2' => Self::_2,
            _ => return None,
        })
    }
}

impl CardTypeJokers {
    fn parse(c: char) -> Option<Self> {
        Some(match c {
            'A' => Self::_A,
            'K' => Self::_K,
            'Q' => Self::_Q,
            'J' => Self::_J,
            'T' => Self::_T,
            '9' => Self::_9,
            '8' => Self::_8,
            '7' => Self::_7,
            '6' => Self::_6,
            '5' => Self::_5,
            '4' => Self::_4,
            '3' => Self::_3,
            '2' => Self::_2,
            _ => return None,
        })
    }
}

type Hand<CardType> = [CardType; 5];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType<CardType> {
    FiveOfAKind(Hand<CardType>),
    FourOfAKind(Hand<CardType>),
    Fullhouse(Hand<CardType>),
    ThreeOfAKind(Hand<CardType>),
    TwoPair(Hand<CardType>),
    OnePair(Hand<CardType>),
    HighCard(Hand<CardType>),
}

impl HandType<CardTypeStandard> {
    fn parse(hand: Hand<CardTypeStandard>) -> Self {
        #[derive(Clone, Copy)]
        struct Count {
            card: CardTypeStandard,
            count: u32,
        }
        let mut counts = [Count {
            card: CardTypeStandard::_2,
            count: 0,
        }; 5];
        for card in hand {
            let hit = counts
                .iter_mut()
                .find(|c| c.card == card || c.count == 0)
                .unwrap();
            hit.card = card;
            hit.count += 1;
        }
        // Sort descending
        counts.sort_by_key(|c| std::cmp::Reverse(c.count));

        match (counts[0].count, counts[1].count) {
            (5, 0) => Self::FiveOfAKind(hand),
            (4, 1) => Self::FourOfAKind(hand),
            (3, 2) => Self::Fullhouse(hand),
            (3, 1) => Self::ThreeOfAKind(hand),
            (2, 2) => Self::TwoPair(hand),
            (2, 1) => Self::OnePair(hand),
            (1, 1) => Self::HighCard(hand),
            _ => unreachable!("High card should always be valid"),
        }
    }
}

impl HandType<CardTypeJokers> {
    fn parse(hand: Hand<CardTypeJokers>) -> Self {
        #[derive(Clone, Copy)]
        struct Count {
            count: u32,
            card: CardTypeJokers,
        }
        let mut counts = [Count {
            card: CardTypeJokers::_J,
            count: 0,
        }; 5];
        for card in hand {
            let hit = counts
                .iter_mut()
                .find(|c| c.card == card || c.count == 0)
                .unwrap();
            hit.card = card;
            hit.count += 1;
        }

        // Take out jokers
        let jokers = counts
            .iter_mut()
            .find_map(|c| (c.card == CardTypeJokers::_J).then(|| std::mem::take(&mut c.count)))
            .unwrap_or_default();

        // Sort descending
        counts.sort_by_key(|c| std::cmp::Reverse(c.count));

        // Add jokers to best possible type
        match (counts[0].count + jokers, counts[1].count) {
            (5, 0) => Self::FiveOfAKind(hand),
            (4, 1) => Self::FourOfAKind(hand),
            (3, 2) => Self::Fullhouse(hand),
            (3, 1) => Self::ThreeOfAKind(hand),
            (2, 2) => Self::TwoPair(hand),
            (2, 1) => Self::OnePair(hand),
            (1, 1) => Self::HighCard(hand),
            _ => unreachable!("High card should always be valid"),
        }
    }
}

#[derive(Debug)]
struct Bid<CardType> {
    hand: HandType<CardType>,
    bid: usize,
}

#[test]
fn task1() {
    type CardType = CardTypeStandard;

    let mut data = open_first(&[
        "src/day07/input.txt",
        "src/day07/sample.txt",
        //
    ])
    .unwrap()
    .lines()
    .map(|line| {
        let (hand, bid) = line.split_once(' ').expect("hand-bid separation");
        let bid = bid.parse::<usize>().unwrap();
        let hand: Hand<CardType> = hand
            .chars()
            .map(|c| CardType::parse(c).expect("valid card"))
            .collect::<Vec<_>>()
            .try_into()
            .expect("A valid hand of 5 cards");
        let hand = HandType::<CardType>::parse(hand);
        return Bid { hand, bid };
    })
    .collect::<Vec<_>>();

    // println!("{:#?}", data);

    data.sort_by_key(|data| std::cmp::Reverse(data.hand));

    // println!("{:#?}", data);

    let result: usize = data.iter().enumerate().map(|(i, b)| b.bid * (i + 1)).sum();
    println!("Result: {}", result);
}

#[test]
fn task2() {
    type CardType = CardTypeJokers;

    let mut data = open_first(&[
        "src/day07/input.txt",
        "src/day07/sample.txt",
        //
    ])
    .unwrap()
    .lines()
    .map(|line| {
        let (hand, bid) = line.split_once(' ').expect("hand-bid separation");
        let bid = bid.parse::<usize>().unwrap();
        let hand: Hand<CardType> = hand
            .chars()
            .map(|c| CardType::parse(c).expect("valid card"))
            .collect::<Vec<_>>()
            .try_into()
            .expect("A valid hand of 5 cards");
        let hand = HandType::<CardType>::parse(hand);
        return Bid { hand, bid };
    })
    .collect::<Vec<_>>();

    // println!("{:#?}", data);

    data.sort_by_key(|data| std::cmp::Reverse(data.hand));

    // println!("{:#?}", data);

    let result: usize = data.iter().enumerate().map(|(i, b)| b.bid * (i + 1)).sum();
    println!("Result: {}", result);
}
