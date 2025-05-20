#![allow(dead_code)]

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let v1 = vec![1, 2, 3];

        // In Rust, iterators are lazy i.e. not computed until consumed
        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("Got: {val}");
        }
    }

    #[test]
    fn example2() {
        let v1 = vec![1, 2, 3];

        // When calling the next method on an iterator, we change the
        // internal state that the iterator uses to keep track of where
        // it is in the sequence. Thus, we need the mut keyword.
        let mut v1_iter = v1.iter();

        // The values we get from the calls to next are immutable references
        // to the values in the vector. If we want to take ownership of the
        // values, we can call into_iter instead of iter. Similarly, if we want
        // to iterate over mutable refs, we can call iter_mut instead of iter.
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        // Methods that call next are called consuming adapters because calling
        // them uses up the iterator. One example is the sum method, which takes
        // ownership of the iterator and iterates through the items by repeatedly
        // calling next, thus consuming the iterator. As it iterates through, it
        // adds each item to a running total and returns the total when iteration
        // is complete.

        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn map_iterator() {
        let v1: Vec<i32> = vec![1, 2, 3];

        // The closure here never gets called. Remember, iterator adapters are lazy.
        // v1.iter().map(|x| x + 1);

        // We need to add the collect() method to conume the iterator and collect all
        // resultant values.
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
