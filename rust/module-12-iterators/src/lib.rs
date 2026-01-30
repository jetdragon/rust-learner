pub fn iterator_basics_example() {
    let v = vec![1, 2, 3];
    let mut iter = v.into_iter();
    while let Some(val) = iter.next() {
        println!("{}", val);
    }
}

pub fn consumer_adapter_example() {
    let v = vec![1, 2, 3, 4, 5];
    let sum: i32 = v.iter().sum();
    println!("Sum: {}", sum);
}

pub fn iterator_adapter_example() {
    let v = vec![1, 2, 3, 4, 5];
    let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);
}

pub fn custom_iterator_example() {
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::new();
    while let Some(val) = counter.next() {
        println!("{}", val);
    }
}
