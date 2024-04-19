// Implementation on Boyer-Moore string searching algorithm

pub struct BoyerMoore;

const MAX_U8: usize = u8::MAX as usize;

impl BoyerMoore {
    pub fn find_match(data: &[u8], query: &[u8]) -> Option<usize> {
        let query_size = query.len();
        let mut i = query_size - 1;
        let mut j = i;
        let delta1 = BoyerMoore::preprocess_delta1(query);

        if query_size > data.len() {
            return None;
        }

        while i < data.len() {
            if data[i] == query[j] {
                if j == 0 {
                    return Some(i);
                }
                i -= 1;
                j -= 1;
            } else {
                let shift = delta1[data[i as usize] as usize];

                if shift < j {
                    i += query_size - j;
                } else {
                    i += shift;
                }

                j = query_size - 1;
            }
        }

        return None;
    }

    pub fn preprocess_delta1(query: &[u8]) -> [usize; MAX_U8 + 1] {
        let query_size = query.len();
        let mut delta1 = [query_size; MAX_U8 + 1];

        for idx in 0..query_size {
            delta1[query[idx] as usize] = query_size - idx - 1;
        }

        delta1
    }

    fn preprocess_delta2(query: &[u8]){
        todo!();
    }
}
