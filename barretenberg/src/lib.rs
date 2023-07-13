use std::{time::Instant, ffi::c_void};

extern "C" {
    fn create_input(size: usize) -> *mut c_void;
    fn create_prover_factory() -> *mut c_void;
    fn commit(input: *mut c_void, n: usize, prover_factory: *mut c_void);
    fn free_crs(ptr: *mut c_void);
    fn free_input(ptr: *mut c_void);
}

pub fn bench(size: usize) -> f64 {
    let mut count = 0;
    let mut duration = 0.0;

    let input = unsafe { create_input(size) };
    let prover_factory = unsafe { create_prover_factory() };

    loop {
        count += 1;
        let now = Instant::now();

        unsafe { commit(input, size, prover_factory) };

        duration += now.elapsed().as_secs_f64();
        if duration > 5.0 {
            break;
        }
    }

    unsafe { free_crs(prover_factory) };
    unsafe { free_input(input)};
    duration / count as f64
}

pub fn run(max_exponent: usize) {
    println!("size,duration,throughput");
    for i in 10..max_exponent {
        let size = 1_usize << i;
        let duration = bench(256);
        let throughput = size as f64 / duration;
        println!("{size},{duration},{throughput}");

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //
    }
}
