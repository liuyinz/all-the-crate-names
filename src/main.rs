use rayon::prelude::*;

fn main() {
    let index = crates_index::Index::new_cargo_default().unwrap();
    index.crates_parallel().for_each(|crate_| {
        println!("{}", crate_.unwrap().name());
    });
}
