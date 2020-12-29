#![crate_name = "permutations"]
use std::ops::Range;
use std::fmt::Debug;

// The Permutation is a Vector of elements to be sequentially permutated.
// At each step the element at start index is swapped with the element at step index.

struct Permutation<T: Debug> {
    // The vector of elements.
    elements: Vec<T>,
    // This index is going to be swapped at each step.
    start: usize,
    // The indexes to be swapped with start index at each step.
    steps: Range<usize>,
}

impl<T: Clone + Debug> Permutation<T> {
    fn new(elements: Vec<T>, start: usize) -> Self {
        let steps = start+1..elements.len();
        Self {
            start,
            elements,
            steps,
        }
    }

    fn update(&mut self, elements: Vec<T>) {
        self.steps = self.start +1.. self.elements.len();
        self.elements = elements;
    }
}


// When next() is called in Permutation struct, we are calling next() in steps field.
//  - if there is another step then:
//      <> we are swapping the elements between start and step
//      <> and the permuted elements are returned in Some()
//  - if there isn't another step, None is returned.

impl<T: Clone + Debug> Iterator for Permutation<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        if let Some(idx) = self.steps.next() {
            self.elements.swap(self.start, idx);
            Some(self.elements.to_vec())
        } else {
            None
        }
    }
}

/// Permutations is a collection of elements to be permuted.
pub struct Permutations<T: Debug> {
    permutation: Vec<Permutation<T>>,
    current_level: usize,
    k: usize,
}

pub trait Combinatorial: Iterator {
    fn permutations(self, k: usize) -> Option<Permutations<Self::Item>>
        where
            Self: Sized,
            Self::Item: Clone + Debug
    {

        let elements: Vec<Self::Item> = self.collect();
        if k==0 || elements.is_empty() || k > elements.len() {
            return None;
        }
        let mut levels: Vec<Permutation<Self::Item>> = Vec::new();
        for i in 0..k - 1 {
            levels.push(Permutation::new(elements.to_vec(), i));
        }

        let mut serie = Permutation::new(elements.to_vec(), k - 1);
        serie.steps = serie.start..elements.len();
        levels.push(serie);

        Some(Permutations {
            permutation: levels,
            current_level: k - 1,
            k,
        })
    }
}


impl<T: ?Sized> Combinatorial for T where T: Iterator { }

impl<T: Clone + Debug> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        loop {

            if let Some(mut permut) = self.permutation[self.current_level].next()  {
                permut.truncate(self.k);
                return Some(permut);
            } else {

                loop {
                    if self.current_level == 0 {
                        return None;
                    } else {
                        self.current_level -= 1;
                        if let Some(mut permut) = self.permutation[self.current_level].next()  {
                            for idx in self.current_level+1..self.k {
                                self.permutation[idx].update(permut.to_vec());
                            }
                            self.current_level = self.k -1;
                            permut.truncate(self.k);
                            return Some(permut);
                        }
                    }
                }
            }
        }
    }
}