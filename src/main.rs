mod common;
mod p1;
mod p2;
mod p3;
mod p4;
mod p5;
mod p9;
mod p19;
mod p20;
mod p21;
mod p23;
mod p24;
mod p61;
mod p82;
mod p83;
mod p94;
mod p100;
mod p101;
mod p104;
mod p108;
mod p110;
mod p111;
mod p112;
mod p206;
mod p234;
mod p224;
mod p363;
mod p495;
mod p621;
mod p766;
mod p2432;
mod solutions;
mod all_your_base;
mod allergies;
mod binary_search;
mod dot_dsl;
mod alphamettics;
mod word_counter;
mod complex;
mod trait_objects;
mod dyn_trait;
mod pig_latin;
mod grade_school;
mod pascals_triangle;
mod isbn_verifier;
mod borrowing_functions;
mod borrowing_match;
mod return_func_ref;
mod box_basics;
mod recusive_type;
mod linked_list;
mod rc_weak;
mod cell;
mod refcell_basics;
mod cell_cache;
mod result_basics_fixed;
mod panic_unwinding;
mod catch_unwind;
mod let_ref_mut;
mod match_ref;

fn main() -> Result<(), &'static str> {
    let s = vec!["apple", "mango", "banana"];
    let fourth = s.get(4).ok_or("I got only 3 fruits")?;
    Ok(())
}
