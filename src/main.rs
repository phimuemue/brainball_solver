extern crate fnv;
extern crate rand;

use fnv::FnvHashMap as HashMap;
use fnv::FnvHashSet as HashSet;

static N_BITS_PER_CELL : usize = 5;
static N_CELL_MASK : u64 = 0b11111u64;
#[inline]
fn extract_cell(n_numbers: u64, i_cell: usize) -> u64 {
    let n_bits = i_cell*N_BITS_PER_CELL;
    (n_numbers & (N_CELL_MASK << n_bits)) >> n_bits
}

include!("flipping_tables.in");

#[derive(PartialEq, Eq, Clone, Hash)]
struct SBall {
    n_cells : u64,
}

static N_MASK_4 : u64 = 0b11111_11111_11111_11111;
static N_MASK_3 : u64 = 0b11111_11111_11111;

static AI_SHIFT_INDEX_PRI_4 : [usize; 6] = [5*0,5*1,5*2,5*6,5*7,5*8];
static AI_SHIFT_INDEX_PRI_3 : [usize; 6] = [5*7,5*8,5*9,5*0,5*1,5*2];
static AN_INVERTED_FLIP_MASK_PRI : [u64; 6] = [ // == (N_MASK_4 << AI_SHIFT_INDEX_PRI_4[i]) | (N_MASK_3 << AI_SHIFT_INDEX_PRI_3[i])
    !((0b11111_11111_11111_11111 << (5*0)) | (0b11111_11111_11111 << (5*7))),
    !((0b11111_11111_11111_11111 << (5*1)) | (0b11111_11111_11111 << (5*8))),
    !((0b11111_11111_11111_11111 << (5*2)) | (0b11111_11111_11111 << (5*9))),
    !((0b11111_11111_11111_11111 << (5*6)) | (0b11111_11111_11111 << (5*0))),
    !((0b11111_11111_11111_11111 << (5*7)) | (0b11111_11111_11111 << (5*1))),
    !((0b11111_11111_11111_11111 << (5*8)) | (0b11111_11111_11111 << (5*2))),
];

static AI_SHIFT_INDEX_SEC_LO : [usize; 7] = [5*0,5*1,5*2,5*6,5*7,5*8,5*9];
static AI_SHIFT_INDEX_SEC_HI : [usize; 7] = [5*7,5*8,5*9,5*0,5*1,5*2,5*3];
static AN_INVERTED_FLIP_MASK_SEC : [u64; 7] = [ // == (N_MASK_3 << AI_SHIFT_INDEX_SEC_LO[i]) | (N_MASK_3 << AI_SHIFT_INDEX_SEC_HI[i]);
    !((0b11111_11111_11111 << (5*0)) | (0b11111_11111_11111 << (5*7))),
    !((0b11111_11111_11111 << (5*1)) | (0b11111_11111_11111 << (5*8))),
    !((0b11111_11111_11111 << (5*2)) | (0b11111_11111_11111 << (5*9))),
    !((0b11111_11111_11111 << (5*6)) | (0b11111_11111_11111 << (5*0))),
    !((0b11111_11111_11111 << (5*7)) | (0b11111_11111_11111 << (5*1))),
    !((0b11111_11111_11111 << (5*8)) | (0b11111_11111_11111 << (5*2))),
    !((0b11111_11111_11111 << (5*9)) | (0b11111_11111_11111 << (5*3))),
];

trait TNum {
    type Prev: TNum;
    #[inline(always)]
    fn value() -> usize;
}

macro_rules! impl_num {($name: ident, $val: expr, $prev: ident) => {
    struct $name {}
    impl TNum for $name {
        type Prev = $prev;
        fn value() -> usize { $val }
    }
}}

impl_num!(SNum0, 0, SNum0); // hack: 0 is its own previous number
impl_num!(SNum1, 1, SNum0);
impl_num!(SNum2, 2, SNum1);
impl_num!(SNum3, 3, SNum2);
impl_num!(SNum4, 4, SNum3);
impl_num!(SNum5, 5, SNum4);
impl_num!(SNum6, 6, SNum5);
impl_num!(SNum7, 7, SNum6);
impl_num!(SNum8, 8, SNum7);
impl_num!(SNumUNDEFINED, 9999, SNumUNDEFINED); // another hack

impl SBall {
    fn new() -> SBall {
        SBall {
            n_cells : 0b11000_10110_10100_10010_10000_01110_01100_01010_01000_00110_00100_00010,
        }
        //let ball = SBall {
        //    n_cells : {
        //        let mut n_numbers = 0u64;
        //        for (i, n_num) in an.iter().enumerate() {
        //            assert!(*n_num<0b1111);
        //            n_numbers |= (*n_num as u64) << (i*4);
        //        }
        //        n_numbers
        //    },
        //    ab: ab.to_vec(),
        //};
        //println!("{:x}", ball.n_numbers);
        //ball
    }

    //fn count_different_cells(&self, ball: &SBall) -> usize {
    //    let n_cells_diff = self.n_cells ^ ball.n_cells;
    //    (0..12).map(|i_cell| 
    //        if 0==extract_cell(n_cells_diff, i_cell) {
    //            0
    //        } else {
    //            1
    //        }
    //    )
    //    .sum()
    //}

    fn colors_correct(&self) -> bool {
        0 == self.n_cells & 0b00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001
    }

    fn is_solved(&self) -> bool {
        self.n_cells & 0b11110_11110_11110_11110_11110_11110_11110_11110_11110_11110_11110_11110 == 0b11000_10110_10100_10010_10000_01110_01100_01010_01000_00110_00100_00010
    }

    //    ...................................
    // 00 01 02 03 04 05 06 07 08 09 10 11 12
    // -1 00 01 02 03 04 05 06 07 08 09 10 11 (cell indices)
    //    ^^ ^^ ^^ ^^          ^^ ^^ ^^        0
    //       ^^ ^^ ^^ ^^          ^^ ^^ ^^     1
    //          ^^ ^^ ^^ ^^          ^^ ^^ ^^  2
    //    ^^ ^^ ^^          ^^ ^^ ^^ ^^        6
    //       ^^ ^^ ^^          ^^ ^^ ^^ ^^     7
    //          ^^ ^^ ^^          ^^ ^^ ^^ ^^	8
    // 
    //    ...................................
    // 00 01 02 03 04 05 06 07 08 09 10 11 12
    // -1 00 01 02 03 04 05 06 07 08 09 10 11 (cell indices)
    //    ^^ ^^ ^^             ^^ ^^ ^^        3
    //       ^^ ^^ ^^             ^^ ^^ ^^     4
    //          ^^ ^^ ^^             ^^ ^^ ^^  5
    //    ^^ ^^ ^^          ^^ ^^ ^^           9
    //       ^^ ^^ ^^          ^^ ^^ ^^        10
    //          ^^ ^^ ^^          ^^ ^^ ^^     11
    //             ^^ ^^ ^^          ^^ ^^ ^^  12

    #[inline(always)]
    fn primary_flip(&mut self, n_flip: usize) {
        debug_assert!(n_flip<6);
        let n_4_to_be_flipped = (self.n_cells & (N_MASK_4 << AI_SHIFT_INDEX_PRI_4[n_flip])) >> AI_SHIFT_INDEX_PRI_4[n_flip];
        let n_3_to_be_flipped = (self.n_cells & (N_MASK_3 << AI_SHIFT_INDEX_PRI_3[n_flip])) >> AI_SHIFT_INDEX_PRI_3[n_flip];
        self.n_cells = (self.n_cells & AN_INVERTED_FLIP_MASK_PRI[n_flip]) // erase old numbers
            | (AN_FLIPPING_TABLE_4[n_4_to_be_flipped as usize] << AI_SHIFT_INDEX_PRI_4[n_flip]) // install 4 flipped numbers
            | (AN_FLIPPING_TABLE_3[n_3_to_be_flipped as usize] << AI_SHIFT_INDEX_PRI_3[n_flip]); // install 3 flipped numbers
    }

    #[inline(always)]
    fn secondary_flip(&mut self, n_flip: usize) {
        debug_assert!(n_flip<7);
        let n_lo_to_be_flipped = (self.n_cells & (N_MASK_3 << AI_SHIFT_INDEX_SEC_LO[n_flip])) >> AI_SHIFT_INDEX_SEC_LO[n_flip];
        let n_hi_to_be_flipped = (self.n_cells & (N_MASK_3 << AI_SHIFT_INDEX_SEC_HI[n_flip])) >> AI_SHIFT_INDEX_SEC_HI[n_flip];
        self.n_cells = (self.n_cells & AN_INVERTED_FLIP_MASK_SEC[n_flip]) // erase old numbers
            | (AN_FLIPPING_TABLE_3[n_lo_to_be_flipped as usize] << AI_SHIFT_INDEX_SEC_HI[n_flip])
            | (AN_FLIPPING_TABLE_3[n_hi_to_be_flipped as usize] << AI_SHIFT_INDEX_SEC_LO[n_flip]);
    }

    #[inline(always)]
    fn flip(&mut self, n_flip: usize) {
        if n_flip<6 {
            self.primary_flip(n_flip);
        } else {
            assert!(n_flip<13);
            self.secondary_flip(n_flip-6);
        }
    }

    //fn get_num(&self, i: usize) -> usize {
    //    ((self.n_cells & (0b11111u64 << (i*5))) >> (i*5)) as usize
    //}

    fn find_solution<
        NumDepth,
        FnSuccess,
    > (
        &self,
        slcn: &mut [usize],
        fn_success: &mut FnSuccess,
    )
        where
            FnSuccess: FnMut(&SBall, &[usize]) -> bool, // result indicates whether we want to continue
            NumDepth: TNum,
    {
        assert_eq!(NumDepth::value(), slcn.len());
        self.internal_find_solution::<NumDepth, SNumUNDEFINED, SNumUNDEFINED, _>(slcn, fn_success)
    }

    fn internal_find_solution<
        NumDepth,
        NumLastPriFlip,
        NumLastSecFlip,
        FnSuccess,
    > (
        &self,
        slcn: &mut [usize],
        fn_success: &mut FnSuccess,
    )
        where
            FnSuccess: FnMut(&SBall, &[usize]) -> bool, // result indicates whether we want to continue
            NumDepth: TNum,
            NumLastPriFlip: TNum,
            NumLastSecFlip: TNum,
    {
        if !fn_success(self, &slcn[0..slcn.len()-NumDepth::value()]) {
            return;
        }
        if 0==NumDepth::value() {
            return;
        }
        macro_rules! impl_sec_flip{($num: ident) => {
            if NumLastSecFlip::value()!=$num::value() {
                let mut ball_next = self.clone();
                ball_next.secondary_flip($num::value());
                {
                    assert!(slcn.len() - NumDepth::value() < slcn.len());
                    let n_slcn_len = slcn.len();
                    slcn[n_slcn_len - NumDepth::value()] = 6 + $num::value();
                }
                ball_next.internal_find_solution::<NumDepth::Prev,SNumUNDEFINED,$num,_>(slcn, fn_success);
            }
        }}
        impl_sec_flip!(SNum0);
        impl_sec_flip!(SNum1);
        impl_sec_flip!(SNum2);
        impl_sec_flip!(SNum3);
        impl_sec_flip!(SNum4);
        impl_sec_flip!(SNum5);
        impl_sec_flip!(SNum6);

        macro_rules! impl_pri_flip{($num: ident) => {
            if NumLastPriFlip::value()!=$num::value() {
                let mut ball_next = self.clone();
                ball_next.primary_flip($num::value());
                {
                    assert!(slcn.len() - NumDepth::value() < slcn.len());
                    let n_slcn_len = slcn.len();
                    slcn[n_slcn_len - NumDepth::value()] = $num::value();
                }
                ball_next.internal_find_solution::<NumDepth::Prev,$num,SNumUNDEFINED,_>(slcn, fn_success);
            }
        }}
        impl_pri_flip!(SNum0);
        impl_pri_flip!(SNum1);
        impl_pri_flip!(SNum2);
        impl_pri_flip!(SNum3);
        impl_pri_flip!(SNum4);
        impl_pri_flip!(SNum5);
    }
}

fn print_cell(n_cell: u64) {
    let b_sign = 0b1==(n_cell & 0b1);
    let n_num = (n_cell & (0b1111 << 1)) >> 1;
    if b_sign {
        print!("-");
    }
    print!("{},", n_num);
}

fn print_ball(ball: &SBall) {
    for i in 0..12 {
        print_cell(extract_cell(ball.n_cells, i));
    }
    println!("");
}

fn main() {
    if false {
        let ball = SBall::new();
        let mut vecovecn : Vec<Option<Vec<usize>>> = Vec::new();
        for _ in 0..4096 {
            vecovecn.push(None);
        }
        let mut an = [ 9999, 9999, 9999, 9999, 9999, 9999, 9999, 9999, ];
        assert_eq!(an.len(), SNum8::value());
        ball.find_solution::<SNum8,_>(
            &mut an,
            &mut |ball, slcn| {
                //0 == self.n_cells & 0b00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001_00001;
                let mut n_color = 0;
                for i in 0..12 {
                    let n = if 0!=(ball.n_cells & (0b1 << (5 * i))) {1} else {0};
                    n_color = (n_color<<1) | n;
                }
                assert!(n_color<4096);
                if vecovecn[n_color].is_none() || vecovecn[n_color].as_ref().unwrap().len()>slcn.len() {
                    vecovecn[n_color] = Some(slcn.to_vec());
                }
                true
            },
        );
        assert_eq!(vecovecn.iter().filter(|ovecn| ovecn.is_some()).count(), 4096);
        for ovecn in vecovecn.iter() {
            print!("{:?}, //", ovecn.as_ref().unwrap());
            {
                let mut ball = SBall::new();
                for flip in ovecn.as_ref().unwrap().iter() {
                    ball.flip(*flip);
                }
                print_ball(&ball);
            }
        }
        return;
    }

    let ball = { // "input" ball - immutable so that we can always look back what it initially was
        let mut ball = SBall::new();
        for n in 0..10000 {
            ball.flip((4*n+7)%13);
        }
        // generate random configuration
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _i in 0..rng.gen_range(1, 100000) {
            ball.flip(rng.gen_range(0, 13));
        }
        ball
    };
    {
        // try to find optimal solution by looking "from both sides"
        let mut mapballn_flips = HashMap::default(); //with_capacity_and_hasher(3_000_000, Default::default());
        mapballn_flips.insert(SBall::new(), 0);
        let mut an = [ 9999, 9999, 9999, 9999, 9999, 9999, 9999 ];
        assert_eq!(SNum7::value(), an.len());
        SBall::new().find_solution::<SNum7, _>(
            &mut an,
            &mut |ball, slcflip| {
                let mut n_flips = mapballn_flips.entry(ball.clone()).or_insert(slcflip.len());
                if slcflip.len() < *n_flips {
                    *n_flips = slcflip.len();
                }
                true
            }
        );
        let mut opairnvecflip = None;
        ball.find_solution::<SNum7, _>(
            &mut an,
            &mut |ball, slcflip| {
                if let Some(n_flips) = mapballn_flips.get(&ball) {
                    if opairnvecflip.is_none() {
                        opairnvecflip = Some((n_flips, slcflip.to_vec()));
                    } else if n_flips + slcflip.len() < opairnvecflip.as_ref().unwrap().0 + opairnvecflip.as_ref().unwrap().1.len() {
                        opairnvecflip = Some((n_flips, slcflip.to_vec()));
                    }
                    assert!(opairnvecflip.is_some());
                }
                true
            }
        );
        if let Some((n_flips, mut vecflip)) = opairnvecflip {
            let mut ball_playback = ball.clone();
            for flip in vecflip.iter() {
                ball_playback.flip(*flip);
            }
            let mut ovecflip_solve_playback = None;
            SBall::new().find_solution::<SNum7, _>(
                &mut an,
                &mut |ball, slcflip| {
                    if *n_flips==slcflip.len() && ball.n_cells==ball_playback.n_cells {
                        ovecflip_solve_playback = Some(slcflip.iter().cloned().rev().collect::<Vec<_>>());
                        false
                    } else {
                        true
                    }
                }
            );
            assert!(ovecflip_solve_playback.is_some());
            for flip in ovecflip_solve_playback.unwrap() {
                vecflip.push(flip);
            }
            println!("Solved in {} moves", vecflip.len());
            let mut ball_playback = ball.clone();
            for flip in vecflip {
                print_ball(&ball_playback);
                ball_playback.flip(flip);
            }
            print_ball(&ball_playback);
            return;
        }
    }
    print_ball(&ball);
    println!("Trying to establish same colors...");
    let mut ovecflip_solve_colors_even = None;
    let mut ovecflip_solve_colors_odd = None;
    let mut an = [ 9999, 9999, 9999, 9999, 9999, 9999, 9999, 9999, ];
    assert_eq!(SNum8::value(), an.len());
    ball.find_solution::<SNum8,_>(
        &mut an,
        &mut |ball, slcn| {
            if ball.colors_correct() {
                if 0==slcn.len()%2 {
                    ovecflip_solve_colors_even = Some(slcn.to_vec());
                } else {
                    ovecflip_solve_colors_odd = Some(slcn.to_vec());
                }
                ovecflip_solve_colors_even.is_none() || ovecflip_solve_colors_odd.is_none()
            } else {
                true
            }
        },
    );
    assert!(ovecflip_solve_colors_even.is_some() && ovecflip_solve_colors_odd.is_some());
    println!("Same colors established");
    let mut ball_playback = ball.clone();
    let mut vecn_solution = Vec::new();
    for n_flip in ovecflip_solve_colors_even.unwrap().iter().cloned() {
        print!("{:>width$} : ", n_flip, width=2);
        ball_playback.flip(n_flip);
        vecn_solution.push(n_flip);
        print_ball(&ball_playback);
    }
    let aan_permutation = [
        [5, 1, 4, 0, 4, 5], // [1, 2, 0]
        [2, 1, 5, 2, 5, 2], // [2, 3, 1]
        [3, 4, 1, 4, 0, 3], // [3, 4, 2]
        [4, 5, 2, 5, 1, 4], // [4, 5, 3]
        [5, 9, 2, 9, 2, 5], // [5, 6, 4]
        [3, 8, 11, 8, 12, 3], // [6, 7, 5]
        [4, 3, 0, 4, 0, 4], // [7, 8, 6]
        [5, 4, 1, 5, 1, 5], // [8, 9, 7]
        [3, 0, 3, 0, 4, 3], // [9, 10, 8]
        [4, 1, 4, 1, 5, 4], // [10, 11, 9]
    ];
    while !ball_playback.is_solved() {
        for i in 1..11 { // install numbers one after another
            let i_desired_pos = (i - 1) as usize;
            let mut n_actual_pos = (0..12)
                .find(|&i_cell| extract_cell(ball_playback.n_cells, i_cell)>>1 == i)
                .unwrap();
            assert!(i_desired_pos <= n_actual_pos);
            while n_actual_pos!=i_desired_pos {
                if n_actual_pos==1 {
                    while n_actual_pos!=i_desired_pos {
                        for n_flip in aan_permutation[0].iter().rev() {
                            ball_playback.flip(*n_flip);
                            vecn_solution.push(*n_flip);
                        }
                        n_actual_pos = (0..12)
                            .find(|&i_cell| extract_cell(ball_playback.n_cells, i_cell)>>1 == i)
                            .unwrap();
                    }
                } else {
                    for n_flip in aan_permutation[std::cmp::max(i_desired_pos, n_actual_pos-2)].iter().rev() {
                        ball_playback.flip(*n_flip);
                        vecn_solution.push(*n_flip);
                    }
                    n_actual_pos = (0..12)
                        .find(|&i_cell| extract_cell(ball_playback.n_cells, i_cell)>>1 == i)
                        .unwrap();
                }
            }
        }
        if !ball_playback.is_solved() {
            ball_playback = ball.clone();
            vecn_solution.clear();
            for n_flip in ovecflip_solve_colors_odd.clone().unwrap().iter().cloned() {
                ball_playback.flip(n_flip);
                vecn_solution.push(n_flip);
            }
            assert!(ball_playback.colors_correct());
        }
    }
    let mut ball_test = ball.clone();
    for n_flip in vecn_solution.iter() {
        ball_test.flip(*n_flip);
    }
    assert!(ball_test.is_solved());

    let mut n_old_length = vecn_solution.len();
    let mut vecflip_solution_compressed = compress_solution(&vecn_solution);
    while vecflip_solution_compressed.len() < n_old_length {
        n_old_length = vecflip_solution_compressed.len();
        vecflip_solution_compressed = compress_solution(&vecflip_solution_compressed);
    }

    println!("Solved in {} ({}) moves", vecflip_solution_compressed.len(), vecn_solution.len());
    let mut ball_test = ball.clone();
    for flip in vecflip_solution_compressed {
        print_ball(&ball_test);
        ball_test.flip(flip);
    }
    print_ball(&ball_test);
}

fn compress_solution(vecn_solution: &Vec<usize>) -> Vec<usize> {
    let mut setball = HashSet::default();
    setball.insert(SBall::new());
    for i_lo in 0..vecn_solution.len() {
        let mut ball = SBall::new();
        for i_hi in i_lo..vecn_solution.len() {
            ball.flip(vecn_solution[i_hi]);
            setball.insert(ball.clone());
        }
    }
    let mut mapballvecflip : HashMap<_, Vec<_>> = HashMap::default();
    let mut an = [ 9999, 9999, 9999, 9999, 9999, 9999, 9999, 9999, ];
    assert_eq!(SNum8::value(), an.len());
    SBall::new().find_solution::<SNum8,_>(
        &mut an,
        &mut |ball, vecflip| {
            if setball.contains(&ball) {
                mapballvecflip.insert(ball.clone(), vecflip.to_vec());
            }
            true // always continue
        },
    );
    println!("Found {} entries", mapballvecflip.len());
    let mut veccompress = Vec::new();
    for i in 0..vecn_solution.len() {
        veccompress.push((
            i+1, // single element, initially no compression
            vecn_solution.len() - i, // uncompressed length
        ));
    }
    for i_lo in (0..(vecn_solution.len()-1)).rev() {
        for i_hi in (i_lo+1)..vecn_solution.len() {
            let ball_to_be_compressed = {
                let mut ball = SBall::new();
                for flip in vecn_solution[i_lo..i_hi].iter() {
                    ball.flip(*flip);
                }
                ball
            };
            if let Some(vecflip_compress) = mapballvecflip.get(&ball_to_be_compressed) {
                let n_compressed_len = vecflip_compress.len() + if i_hi==vecn_solution.len() {0} else {veccompress[i_hi].1};
                if veccompress[i_lo].1 > n_compressed_len {
                    veccompress[i_lo].0 = i_hi;
                    veccompress[i_lo].1 = n_compressed_len;
                }
                //if vecflip_compress.len() < veccompress[i_lo].1 {
                //    println!("[{}..{}] {:?} could be compressed to {:?}", i_lo, i_hi, vecn_solution[i_lo..i_hi].to_vec(), vecflip_compress);
                //}
            }
        }
    }
    let mut vecflip_solution_compressed = Vec::new();
    let mut i_compress = 0;
    while i_compress<vecn_solution.len() {
        if veccompress[i_compress].0==i_compress+1 {
            vecflip_solution_compressed.push(vecn_solution[i_compress]);
        } else {
            let ball_to_be_compressed = {
                let mut ball = SBall::new();
                for flip in vecn_solution[i_compress..veccompress[i_compress].0].iter() {
                    ball.flip(*flip);
                }
                ball
            };
            let vecflip_compress = mapballvecflip.get(&ball_to_be_compressed).unwrap();
            for flip in vecflip_compress {
                vecflip_solution_compressed.push(*flip);
            }
        }
        i_compress = veccompress[i_compress].0;
    }
    vecflip_solution_compressed
}
