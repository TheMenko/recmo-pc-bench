use std::{time::Instant, ffi::c_void};

extern "C" {
    fn create_composer(circuit_size: usize) -> *mut c_void;
    fn commit(composer: *mut c_void, length: usize);
    fn free_composer(composer: *mut c_void);
}

pub fn bench(circuit_size: usize) -> f64 {
    let mut count = 0;
    let mut duration = 0.0;

    let composer = unsafe { create_composer(circuit_size) };

    loop {
        count += 1;
        let now = Instant::now();

        unsafe { commit(composer, 1 << 4) };

        duration += now.elapsed().as_secs_f64();
        if duration > 5.0 {
            break;
        }
    }

    unsafe { free_composer(composer) };
    duration / count as f64
}

pub fn run(max_exponent: usize) {
    println!("size,duration,throughput");
    for i in 10..max_exponent {
        let size = 1_usize << i;
        let duration = bench(i);
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
