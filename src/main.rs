extern crate fnv;
extern crate rand;

use fnv::FnvHashMap as HashMap;
use fnv::FnvHashSet as HashSet;
//use rand::Rng;

static N_BITS_PER_CELL : usize = 5;
static N_CELL_MASK : u64 = 0b11111u64;
#[inline]
fn extract_cell(n_numbers: u64, i_cell: usize) -> u64 {
    let n_bits = i_cell*N_BITS_PER_CELL;
    (n_numbers & (N_CELL_MASK << n_bits)) >> n_bits
}
#[inline]
fn set_cell_from_0(n_numbers: u64, i_cell: usize, n_cell: u64) -> u64 {
    debug_assert_eq!(0, extract_cell(n_numbers, i_cell));
    n_numbers | (n_cell << (i_cell*N_BITS_PER_CELL))
}
fn flip_table_entry(n_numbers: u64, n_count: usize) -> u64 {
    let mut n_flipped = 0u64;
    for i_source in 0..n_count {
        let i_dest = n_count-1-i_source;
        let n_source = extract_cell(n_numbers, i_source) ^ 0b1; // XOR to flip color
        n_flipped = set_cell_from_0(n_flipped, i_dest, n_source);
    }
    n_flipped
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

    fn find_solution<FnPred, FnSuccess> (
        &self,
        n_depth: usize,
        mapballn_depth: &mut HashMap<SBall, usize>,
        vecn: &mut Vec<usize>,
        fn_pred: &FnPred,
        fn_success: &mut FnSuccess
    )
        where
            FnPred: Fn(&SBall) -> bool,
            FnSuccess: FnMut(&SBall, &Vec<usize>) -> bool, // result indicates whether we want to continue
    {
        if 7<n_depth {
            return;
            //return None;
        }
        if fn_pred(self) {
            if !fn_success(self, vecn) {
                return;
            }
        }
        if let Some(n_depth_ball_already_searched) = mapballn_depth.get(&self) {
            if n_depth_ball_already_searched <= &n_depth {
                return /*None*/;
            }
        }
        for i in 0..7 {
            let mut ball_next = self.clone();
            ball_next.secondary_flip(i);
            vecn.push(6+i);
            ball_next.find_solution(n_depth+1, mapballn_depth, vecn, fn_pred, fn_success);
            //if let Some(vecnSolution) = self.find_solution(n_depth+1, mapballn_depth, vecn) {
            //    return Some(vecnSolution);
            //}
            vecn.pop().unwrap();
        }
        for i in 0..6 {
            let mut ball_next = self.clone();
            ball_next.primary_flip(i);
            vecn.push(i);
            ball_next.find_solution(n_depth+1, mapballn_depth, vecn, fn_pred, fn_success);
            //if let Some(vecnSolution) = self.find_solution(n_depth+1, mapballn_depth, vecn) {
            //    return Some(vecnSolution);
            //}
            vecn.pop().unwrap();
        }
        mapballn_depth.insert(self.clone(), n_depth);
        //return None;
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
    if false { // TODO clap
        // 4-number flips
        println!("static AN_FLIPPING_TABLE_4 : [u64; 0b11111_11111_11111_11111+1] = [");
        for n_numbers in 0..0b11111_11111_11111_11111+1 {
            let n_output = flip_table_entry(n_numbers, 4);
            print!("    0b{:b}, // ", n_output); // TODO nicer output (20 digits separated using underscores)
            let print_4_nums = |n_4_numbers| {
                print!("(");
                for i in 0..4 {
                    let n_cell = extract_cell(n_4_numbers, i);
                    print_cell(n_cell);
                }
                print!(")");
            };
            print_4_nums(n_numbers);
            print!(" => ");
            print_4_nums(n_output);
            println!("");
        }
        println!("];");
        // 3-number flips
        println!("static AN_FLIPPING_TABLE_3 : [u64; 0b11111_11111_11111+1] = [");
        for n_numbers in 0..0b11111_11111_11111+1 {
            let n_output = flip_table_entry(n_numbers, 3);
            print!("    0b{:b}, // ", n_output); // TODO nicer output (20 digits separated using underscores)
            let print_3_nums = |n_3_numbers| {
                print!("(");
                for i in 0..3 {
                    let n_cell = extract_cell(n_3_numbers, i);
                    let b_sign = 0b1==(n_cell & 0b1);
                    let n_num = (n_cell & (0b1111 << 1)) >> 1;
                    if b_sign {
                        print!("-");
                    }
                    print!("{},", n_num);
                }
                print!(")");
            };
            print_3_nums(n_numbers);
            print!(" => ");
            print_3_nums(n_output);
            println!("");
        }
        println!("];");
        return;
    }

    let ball = { // "input" ball - immutable so that we can always look back what it initially was
        let mut ball = SBall::new();
        for n in 0..10000 {
            ball.flip((2*n+6)%13);
        }
        // generate random configuration
        //let mut rng = rand::thread_rng();
        //for _i in 0..rng.gen_range(1, 100000) {
        //    ball.flip(rng.gen_range(0, 13));
        //}
        ball
    };
    print_ball(&ball);
    println!("Trying to establish same colors...");
    let mut ovecflip_solve_colors_even = None;
    let mut ovecflip_solve_colors_odd = None;
    ball.find_solution(
        0,
        &mut HashMap::default(),
        &mut Vec::new(),
        &|ball| ball.colors_correct(),
        &mut |ball, vecn| {
            assert!(ball.colors_correct());
            if 0==vecn.len()%2 {
                ovecflip_solve_colors_even = Some(vecn.clone());
            } else {
                ovecflip_solve_colors_odd = Some(vecn.clone());
            }
            ovecflip_solve_colors_even.is_some() || ovecflip_solve_colors_odd.is_some()
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
            print!("{} is at {}, but should be at {}: ", i, n_actual_pos, i_desired_pos);
            print_ball(&ball_playback);
            assert!(i_desired_pos <= n_actual_pos);
            while n_actual_pos!=i_desired_pos {
                if n_actual_pos==1 {
                    while n_actual_pos!=i_desired_pos {
                        for n_flip in aan_permutation[0].iter().rev() {
                            ball_playback.flip(*n_flip);
                            vecn_solution.push(*n_flip);
                            print!("After {}: ", n_flip);
                            print_ball(&ball_playback);
                        }
                        n_actual_pos = (0..12)
                            .find(|&i_cell| extract_cell(ball_playback.n_cells, i_cell)>>1 == i)
                            .unwrap();
                    }
                } else {
                    for n_flip in aan_permutation[std::cmp::max(i_desired_pos, n_actual_pos-2)].iter().rev() {
                        ball_playback.flip(*n_flip);
                        vecn_solution.push(*n_flip);
                        print!("After {}: ", n_flip);
                        print_ball(&ball_playback);
                    }
                    n_actual_pos = (0..12)
                        .find(|&i_cell| extract_cell(ball_playback.n_cells, i_cell)>>1 == i)
                        .unwrap();
                }
                print!("{} is at {}, but should be at {}: ", i, n_actual_pos, i_desired_pos);
                print_ball(&ball_playback);
            }
        }
        if !ball_playback.is_solved() {
            ball_playback = ball.clone();
            vecn_solution.clear();
            println!("Using odd");
            for n_flip in ovecflip_solve_colors_odd.clone().unwrap().iter().cloned() {
                print!("{:>width$} : ", n_flip, width=2);
                ball_playback.flip(n_flip);
                vecn_solution.push(n_flip);
                print_ball(&ball_playback);
            }
            assert!(ball_playback.colors_correct());
        }
    }
    print_ball(&ball_playback);
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

    println!("Solution:   {} moves", vecn_solution.len());
    println!("Comperssed: {} moves", vecflip_solution_compressed.len());

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
    SBall::new().find_solution(
        0,
        &mut HashMap::default(),
        &mut Vec::new(),
        &|_ball| true, // consider all moves
        &mut |ball, vecflip| {
            if setball.contains(&ball) {
                mapballvecflip.insert(ball.clone(), vecflip.clone());
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
