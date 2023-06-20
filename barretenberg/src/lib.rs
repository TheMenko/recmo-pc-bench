use std::{time::Instant, ffi::c_void};

extern "C" {
    fn create_composer(circuit_size: usize) -> *mut c_void;
    fn commit(composer: *mut c_void);
}

pub fn bench(circuit_size: usize) -> f64 {
    let mut count = 0;
    let mut duration = 0.0;

    let composer = unsafe { create_composer(circuit_size) };

    loop {
        count += 1;
        let now = Instant::now();

        unsafe { commit(composer) };

        duration += now.elapsed().as_secs_f64();
        if duration > 5.0 {
            break;
        }
    }

    unsafe { free_input(input) };

    duration / count as f64
}

pub fn run(circuit_size: usize) {
    println!("size,duration,throughput");
    for i in circuit_size {
        let duration = bench(circuit_size);
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
