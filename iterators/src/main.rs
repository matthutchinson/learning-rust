fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn consume_iterator() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    // sum is a consuming iterator, so v1_iter is used up now

    assert_eq!(total, 6);
}

#[test]
fn map_and_collect() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // map is an iterator adaptor, does nothing without chaining or collect
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    // collect consumes the iterator

    assert_eq!(v2, vec![2, 3, 4]);
}

// shoe example, filter and collect, with a closure in filter that captures the local shoe_size
// variable. into_iter is used so iterator with filter has ownership of the shoes Vec

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_shoes_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}


// counter example, implementing next on the Iterator trait
// NOTE: made things pub to avoid warnings
pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

// implementing next (takes mutable self), returns Self::Item in Option (Some, None)
impl Iterator for Counter {
    // Item is an associated `type` for this trait (explained later)
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        // allow count to 5, then always return None
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn counter_calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
    assert_eq!(counter.next(), None);
}

#[test]
fn counter_using_other_iterator_trait_methods() {
    // take the values produced by an instance of Counter, pair them with values produced by
    // another Counter instance after skipping the first value, multiply each pair together, keep
    // only those results that are divisible by 3, and add all the resulting values together

    // NOTE:  the theoretical fifth pair (5, None) is never produced because zip returns None when
    // either of its input iterators return None

    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
