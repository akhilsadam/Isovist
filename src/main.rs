use time;

const E: f64 = 2.71828182845904523536028747135;
const PI: f64 = 3.14159265358979323846264338327;
const A: f64 = -1.0;
const C: f64 = 0.97;
const LIMIT: f64 = 100.0;

fn timestamp() -> f64 {
    let timespec = time::get_time();
    // 1459440009.113178
    let mills: f64 = timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
    mills
}

fn raycast(angle: f64) -> f64 {
    let beta = f64::sin(f64::tan(f64::floor(A*C*angle)/A));
    let r = (1.0f64 + f64::tan(angle-beta).powi(2)).powf(0.5f64);
    let mut out = r;
    if r > LIMIT {
        out = LIMIT;
    }
    out
}

fn isovist() -> f64{

    let mut area = 0.0f64;
    let n = 500000;
    for i in 0..n {
        area += (PI/(n as f64))*raycast(2.0f64*PI*(i as f64)/(n as f64)).powi(2)
    }
    area
}

fn main() { 
    let t = timestamp();
    println!("\n\t{}\t{}","Isovist Area: ", isovist());
    println!("\t{}\t{}", "Time Taken: ", timestamp() - t);
}
