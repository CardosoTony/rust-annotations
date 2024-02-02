// Mode 1
/*
use func_examples::ding;
use func_examples::on_off;
use func_examples::print_array;
use func_examples::print_difference;
use func_examples::print_distance1;
use func_examples::print_distance2;
*/

// Mode 2
use func_examples::{
    ding, on_off, print_array, print_difference, print_distance1, print_distance2,
};

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    print_difference(coords.0, coords.1);

    let coords_arr: [f32; 2] = [coords.0, coords.1];
    print_array(coords_arr);

    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[4]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off((mess.2)[1].0);

    print_distance1(coords);
    print_distance2(coords);
}
