use std::{ops::Add, rc::Rc};

#[derive(Clone)]
enum NumberInner {
    Lit(u8),
    Pair(Number, Number),
}

#[derive(Clone)]
struct Number {
    inner: Rc<NumberInner>,
}

impl Number {
    fn lit(lit: u8) -> Self {
        Self {
            inner: Rc::new(NumberInner::Lit(lit)),
        }
    }

    fn unwrap_lit(&self) -> u8 {
        match self.inner.as_ref() {
            NumberInner::Lit(lit) => *lit,
            NumberInner::Pair(_, _) => panic!("not a literal"),
        }
    }

    fn pair(left: Self, right: Self) -> Self {
        Self {
            inner: Rc::new(NumberInner::Pair(left, right)),
        }
    }

    fn parse(s: &str) -> Self {
        fn go(chars: &mut impl Iterator<Item = char>) -> Number {
            match chars.next().unwrap() {
                '[' => {
                    let left = go(chars);
                    assert_eq!(chars.next(), Some(','));
                    let right = go(chars);
                    assert_eq!(chars.next(), Some(']'));
                    Number::pair(left, right)
                }
                c => Number::lit(c.to_digit(10).unwrap().try_into().unwrap()),
            }
        }
        go(&mut s.trim().chars())
    }

    fn explode(&self) -> Option<Self> {
        fn add_leftmost(n: &Number, carry: u8) -> Number {
            match n.inner.as_ref() {
                NumberInner::Lit(lit) => Number::lit(lit + carry),
                NumberInner::Pair(left, right) => {
                    Number::pair(add_leftmost(left, carry), right.clone())
                }
            }
        }

        fn add_rightmost(n: &Number, carry: u8) -> Number {
            match n.inner.as_ref() {
                NumberInner::Lit(lit) => Number::lit(lit + carry),
                NumberInner::Pair(left, right) => {
                    Number::pair(left.clone(), add_rightmost(right, carry))
                }
            }
        }

        fn go(n: &Number, depth: u8) -> Option<(u8, Number, u8)> {
            match n.inner.as_ref() {
                NumberInner::Lit(_) => None,
                NumberInner::Pair(left, right) => {
                    if depth == 4 {
                        Some((left.unwrap_lit(), Number::lit(0), right.unwrap_lit()))
                    } else {
                        go(left, depth + 1)
                            .map(|(left_carry, n, right_carry)| {
                                (
                                    left_carry,
                                    Number::pair(n, add_leftmost(right, right_carry)),
                                    0,
                                )
                            })
                            .or_else(|| {
                                go(right, depth + 1).map(|(left_carry, n, right_carry)| {
                                    (
                                        0,
                                        Number::pair(add_rightmost(left, left_carry), n),
                                        right_carry,
                                    )
                                })
                            })
                    }
                }
            }
        }

        go(self, 0).map(|(_, n, _)| n)
    }

    fn split(&self) -> Option<Self> {
        match self.inner.as_ref() {
            &NumberInner::Lit(lit) => {
                if lit >= 10 {
                    Some(Number::pair(
                        Number::lit(lit / 2),
                        Number::lit(lit - lit / 2),
                    ))
                } else {
                    None
                }
            }
            NumberInner::Pair(left, right) => {
                if let Some(n) = left.split() {
                    Some(Number::pair(n, right.clone()))
                } else {
                    left.split()
                        .map(|n| Number::pair(n, right.clone()))
                        .or_else(|| right.split().map(|n| Number::pair(left.clone(), n)))
                }
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self.inner.as_ref() {
            NumberInner::Lit(lit) => u32::from(*lit),
            NumberInner::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, other: Self) -> Self::Output {
        let mut res = Self::pair(self, other);
        while let Some(n) = res.explode().or_else(|| res.split()) {
            res = n;
        }
        res
    }
}

fn parse(input: &str) -> impl Iterator<Item = Number> {
    input.trim().lines().map(Number::parse)
}

pub fn part1(input: &str) -> u32 {
    parse(input).reduce(|a, b| a + b).unwrap().magnitude()
}

pub fn part2(input: &str) -> u32 {
    let numbers: Vec<_> = parse(input).collect();
    numbers
        .iter()
        .flat_map(|a| numbers.iter().map(|b| (a.clone() + b.clone()).magnitude()))
        .max()
        .unwrap()
}

pub fn tests() {
    let example = "
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    ";
    assert_eq!(part1(example), 4140);
    assert_eq!(part2(example), 3993);
}
