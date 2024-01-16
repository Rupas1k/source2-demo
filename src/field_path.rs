

// #[derive(Debug)]
// pub struct FieldPathFormatter {
//     bits_per_component: [i64; 7],
//     max_fp_len: usize,
//     clear_mask: [i64; 6],
//     present_bit: [i64; 6],
//     value_shift: [i64; 7],
//     value_mask: [i64; 7],
//     offset: [i64; 7],
//     present_mask: i64,
// }

// impl FieldPathFormatter {
//     pub fn new() -> Self {
//         let mut bit_count = -1;
//         let mut present_mask = 0i64;
//
//         let bits_per_component = [11, 12, 11, 8, 7, 4, 4];
//         let max_fp_len = 7;
//
//         let mut clear_mask = [0; 6];
//         let mut present_bit = [0; 6];
//         let mut value_shift = [0; 7];
//         let mut value_mask = [0; 7];
//         let mut offset = [0; 7];
//
//
//         for i in 0..max_fp_len {
//             bit_count += bits_per_component[i] + 1;
//         }
//
//         let mut cur = bit_count;
//
//         for i in 0..max_fp_len {
//             offset[i] = if i == 0 { 1 } else { 0 };
//             // println!("{}", (-1i64 << 52) & ((1i128 << 63) - 1i128) as i64);
//             // println!("{}", (-1i64 << 52) & ((1i64 << 63) - 1i64));
//             if i != 0 {
//                 // println!("{:?}", clear_mask[i - 1]);
//                 // println!("{} {} {:?}", cur, bit_count, clear_mask);
//                 clear_mask[i - 1] = (-1i64 << cur) & ((1i128 << bit_count) - 1) as i64;
//                 cur -= 1;
//                 present_bit[i - 1] = 1i64 << cur;
//                 present_mask |= 1i64 << cur;
//             }
//             cur -= bits_per_component[i];
//
//             value_shift[i] = cur;
//             value_mask[i] = ((1i64 << bits_per_component[i]) - 1) << cur;
//         }
//
//         FieldPathFormatter {
//             bits_per_component,
//             max_fp_len,
//             clear_mask,
//             present_bit,
//             value_shift,
//             value_mask,
//             offset,
//             present_mask,
//         }
//     }
//
//     pub fn set(&self, id: i64, i: usize, v: i64) -> i64 {
//         id & !self.value_mask[i] | (v + self.offset[i]) << self.value_shift[i]
//     }
//
//     pub fn get(&self, id: i64, i: usize) -> i64 {
//         ((id & self.value_mask[i]) >> self.value_shift[i]) - self.offset[i]
//     }
//
//     pub fn down(&self, id: i64) -> i64 {
//         id | self.present_bit[self.last(id)]
//     }
//
//     pub fn up(&self, id: i64) -> i64 {
//         id & self.clear_mask[self.last(id)]
//     }
//
//     pub fn last(&self, id: i64) -> usize {
//         i64::count_ones(id & self.present_mask) as usize
//     }
// }


pub struct FieldPathFormatter {}

impl FieldPathFormatter {
    // pub const BITS_PER_COMPONENT: [i64; 7] = [11, 12, 11, 8, 7, 4, 4];
    // pub const MAX_FP_LEN: usize = 7;
    pub const CLEAR_MASK: [i64; 6] = [9218868437227405312, 9223371487098961920, 9223372036720558080, 9223372036854513664, 9223372036854774784, 9223372036854775776];
    pub const PRESENT_BIT: [i64; 6] = [2251799813685248, 274877906944, 67108864, 131072, 512, 16];
    pub const VALUE_SHIFT: [i64; 7] = [52, 39, 27, 18, 10, 5, 0];
    pub const VALUE_MASK: [i64; 7] = [9218868437227405312, 2251250057871360, 274743689216, 66846720, 130048, 480, 15];
    pub const OFFSET: [i64; 7] = [1, 0, 0, 0, 0, 0, 0];
    pub const PRESENT_MASK: i64 = 2252074758832656;


    pub fn set(id: &i64, i: usize, v: i64) -> i64 {
        id & !Self::VALUE_MASK[i] | (v + Self::OFFSET[i]) << Self::VALUE_SHIFT[i]
    }

    pub fn get(id: &i64, i: usize) -> i64 {
        ((id & Self::VALUE_MASK[i]) >> Self::VALUE_SHIFT[i]) - Self::OFFSET[i]
    }

    pub fn down(id: &i64) -> i64 {
        id | Self::PRESENT_BIT[Self::last(id)]
    }

    pub fn up(id: &i64, n: usize) -> i64 {
        id & Self::CLEAR_MASK[Self::last(id) - n]
    }

    pub fn last(id: &i64) -> usize {
        // println!("{id}");
        // println!("{}", u64::count_ones((id & Self::PRESENT_MASK) as u64) as usize);
        (id & Self::PRESENT_MASK).count_ones() as usize
    }
}

#[derive(Clone, Debug)]
pub struct FieldPath {
    pub id: i64
}

impl FieldPath {
    pub fn new() -> Self {
        FieldPath {
            id: 0
        }
    }

    pub fn get(&self, i: usize) -> i64 {
        FieldPathFormatter::get(&self.id, i)
    }

    pub fn set(&mut self, i: usize, v: i64) {
        self.id = FieldPathFormatter::set(&self.id, i, v)
    }

    pub fn down(&mut self) {
        self.id = FieldPathFormatter::down(&self.id);
    }

    pub fn last(&self) -> usize{
        FieldPathFormatter::last(&self.id)
    }

    pub fn inc(&mut self, i: usize, n: i64) {
        self.set(i, self.get(i) + n)
    }

    pub fn inc_cur(&mut self, n: i64) {
        self.inc(self.last(), n)
    }

    pub fn up(&mut self, n: usize) {
        self.id = FieldPathFormatter::up(&self.id, n);
    }
}
