use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// TODO can build.rs and main.rs share code nicely?
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

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("src/flipping_tables.in");
    let mut f = File::create(&dest_path).unwrap();

    {
        // 4-number flips
        f.write_all(b"static AN_FLIPPING_TABLE_4 : [u64; 0b11111_11111_11111_11111+1] = [\n").unwrap();
        for n_numbers in 0..0b11111_11111_11111_11111+1 {
            let n_output = flip_table_entry(n_numbers, 4);
            f.write_all(format!("    0b{:b}, // ", n_output).as_bytes()).unwrap(); // TODO nicer output (20 digits separated using underscores)
            //let print_4_nums = |n_4_numbers| {
            //    print!("(");
            //    for i in 0..4 {
            //        let n_cell = extract_cell(n_4_numbers, i);
            //        print_cell(n_cell);
            //    }
            //    print!(")");
            //};
            //print_4_nums(n_numbers);
            //print!(" => ");
            //print_4_nums(n_output);
            f.write_all(b"\n").unwrap();
        }
        f.write_all(b"];\n").unwrap();
        // 3-number flips
        f.write_all(b"static AN_FLIPPING_TABLE_3 : [u64; 0b11111_11111_11111+1] = [\n").unwrap();
        for n_numbers in 0..0b11111_11111_11111+1 {
            let n_output = flip_table_entry(n_numbers, 3);
            f.write_all(format!("    0b{:b}, // ", n_output).as_bytes()).unwrap(); // TODO nicer output (20 digits separated using underscores)
            //let print_3_nums = |n_3_numbers| {
            //    print!("(");
            //    for i in 0..3 {
            //        let n_cell = extract_cell(n_3_numbers, i);
            //        let b_sign = 0b1==(n_cell & 0b1);
            //        let n_num = (n_cell & (0b1111 << 1)) >> 1;
            //        if b_sign {
            //            print!("-");
            //        }
            //        print!("{},", n_num);
            //    }
            //    print!(")");
            //};
            //print_3_nums(n_numbers);
            //print!(" => ");
            //print_3_nums(n_output);
            f.write_all(b"\n").unwrap();
        }
        f.write_all(b"];\n").unwrap();
        return;
    }

}
