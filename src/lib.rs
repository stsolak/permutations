use std::ops::Range;

pub struct Series<T> {
    level: usize,
    elements: Vec<T>,
    steps: Range<usize>,
}

impl<T: Clone> Series<T> {
    fn new(elements: &Vec<T>, level: usize) -> Self {
        Self {
            level,
            elements: elements.to_vec(),
            steps: level+1..elements.len(),
        }
    }

    fn reset(&mut self) {
        self.steps = self.level+1.. self.elements.len();
    }

    fn permute(&mut self) -> Option<Vec<T>> {
        if let Some(idx) = self.steps.next() {
            self.elements.swap(self.level,idx);
            Some(self.elements.to_vec())
        } else {
            None
        }
    }
}

pub struct Permutations<T> {
    permutation: Vec<Series<T>>,
    current_level: usize,
    elements_to_take_size: usize,
}

impl<T: Clone> Permutations<T> {
    pub fn new(elements: &Vec<T>, elements_to_take_size: usize) -> Self {
        let mut levels = Vec::new();
        for i in 0..elements_to_take_size-1 {
            levels.push(Series::new(elements,i));
        }

        let mut serie = Series::new( elements,elements_to_take_size-1);
        serie.steps = serie.level..elements.len() ;
        levels.push(serie);

        Permutations{
            permutation: levels,
            current_level: elements.len()-1,
            elements_to_take_size,
        }
    }
}

impl<T: Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        loop {
            if let Some(mut permut) = self.permutation[self.current_level].permute()  {
                permut.truncate(self.elements_to_take_size);
                return Some(permut);
            } else {
                loop {
                    if self.current_level == 0 {
                        return None;
                    } else {
                        self.permutation[self.current_level].reset();
                        self.current_level -= 1;
                        if let Some(mut permut) = self.permutation[self.current_level].permute()  {
                            for idx in self.current_level..self.elements_to_take_size {
                                self.permutation[idx].elements = permut.to_vec();
                            }
                            self.current_level = self.elements_to_take_size-1;
                            permut.truncate(self.elements_to_take_size);
                            return Some(permut);
                        }
                    }
                }
            }
        }
    }
}