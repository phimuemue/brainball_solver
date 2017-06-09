extern crate fnv;

use fnv::FnvHashMap as HashMap;

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

fn vec_shift_right<T>(vect: &mut Vec<T>, n_shift: usize) {
    let n_len = vect.len();
    assert!(0<n_len);
    for _i in 0..n_shift {
        let t = vect.pop().unwrap();
        vect.insert(0, t);
    }
    assert_eq!(n_len, vect.len());
}

fn vec_shift_left<T>(vect: &mut Vec<T>, n_shift: usize) {
    let n_len = vect.len();
    for _i in 0..n_shift {
        let t = vect.remove(0);
        vect.push(t);
    }
    assert_eq!(n_len, vect.len());
}

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
    fn new(_an: [usize; 13], _ab: [bool; 13]) -> SBall {
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

    fn get_num(&self, i: usize) -> usize {
        ((self.n_cells & (0b11111u64 << (i*5))) >> (i*5)) as usize
    }

    fn find_solution(&mut self, n_depth: usize, mapballn_depth: &mut HashMap<SBall, usize>, vecn: &mut Vec<usize>) -> Option<Vec<usize>> {
        if 11<n_depth {
            return None;
        }
        if let Some(n_depth_ball_already_searched) = mapballn_depth.get(&self) {
            if n_depth_ball_already_searched <= &n_depth {
                return None;
            }
        }
        if self.n_cells == 0b11000_10110_10100_10010_10000_01110_01100_01010_01000_00110_00100_00010 {
            return Some(vecn.clone());
        }
        for i in 0..6 {
            let ball_backup = self.clone();
            self.primary_flip(i);
            vecn.push(i);
            if let Some(vecnSolution) = self.find_solution(n_depth+1, mapballn_depth, vecn) {
                return Some(vecnSolution);
            }
            vecn.pop().unwrap();
            self.n_cells = ball_backup.n_cells; // convert back
        }
        for i in 0..7 {
            let ball_backup = self.clone();
            self.secondary_flip(i);
            vecn.push(6+i);
            if let Some(vecnSolution) = self.find_solution(n_depth+1, mapballn_depth, vecn) {
                return Some(vecnSolution);
            }
            vecn.pop().unwrap();
            self.n_cells = ball_backup.n_cells; // convert back
        }
        mapballn_depth.insert(self.clone(), n_depth);
        return None;
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


    let mut ball = SBall::new(
        [1,2,3,4,5,6,7,8,9,10,11,12,13],
        [true, true, true, true, true, true, true, true, true, true, true, true, true],
    );
    print_ball(&ball);
    ball.primary_flip(0);
    print_ball(&ball);
    ball.secondary_flip(4);
    print_ball(&ball);
    ball.primary_flip(4);
    print_ball(&ball);
    ball.primary_flip(4);
    print_ball(&ball);
    ball.secondary_flip(4);
    print_ball(&ball);
    ball.primary_flip(4);
    print_ball(&ball);
    if let Some(vecn) = ball.clone().find_solution(0, &mut HashMap::default(), &mut Vec::new()) {
        println!("Found solution:");
        let mut ball_playback = ball.clone();
        for n in vecn {
            print_ball(&ball_playback);
            println!("{}", n);
            if n<6 {
                ball_playback.primary_flip(n);
            } else {
                assert!(n<13);
                ball_playback.secondary_flip(n-6);
            }
        }
        print_ball(&ball_playback);
    } else {
        println!("No solution found");
    }
}
