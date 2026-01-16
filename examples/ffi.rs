use std::os::raw::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: c_int,
    y: c_int,
}

unsafe extern "C" {
    fn add(a: c_int, b: c_int) -> c_int;
    fn point_manhattan(p: Point) -> c_int;
}

fn main() {
    unsafe {
        let s = add(3, 4);
        println!("add(3,4) = {}", s);

        let p = Point { x: -5, y: 7 };
        let m = point_manhattan(p);
        println!("point {:?} manhattan = {}", p, m);
    }
}
