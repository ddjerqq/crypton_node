const DEGREES: usize = 360;

#[derive(Debug, Clone)]
pub struct HashRing {
    circle: Vec<usize>,
}

impl HashRing {
    pub fn new(nodes: usize) -> Self {
        let circle = (0..nodes)
            .map(|i| i * DEGREES / nodes)
            .collect();

        return Self {
            circle,
        }
    }

    pub fn get(&self, item: usize) -> usize {
        let item_degree = item % DEGREES;

        let current_degree = {
            let mut current_degree = 0usize;

            for degree in self.circle.iter() {
                if *degree > item_degree {
                    break;
                }

                current_degree = *degree;
            }

            current_degree
        };

        for (idx, degree) in self.circle.iter().enumerate() {
            if current_degree == *degree {
                return idx;
            }
        }

        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_ring() {
        let ring = HashRing::new(3);

        for i in 0..DEGREES {
            println!("{} {}", i, ring.get(i));
        }
    }
}