#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sma() {
        let mut close_prices: Vec<TA_Real> = Vec::with_capacity(400);
        let mut out: Vec<TA_Real> = Vec::with_capacity(400);;
        let mut out_begin: TA_Integer = 0;
        let mut out_size: TA_Integer = 0;

        for i in 0..400 {
            close_prices.push(i as TA_Real);
        }
        unsafe {
            let ret_code = TA_MA(0, 399, close_prices.as_mut_ptr(), 30, TA_MAType::TA_MAType_SMA,
                                 &mut out_begin, &mut out_size, out.as_mut_ptr());
            for i in 0..out_size {
                println!("Day {} = {}", out_begin + i, out.get_unchecked(i as usize));
            }
        }
    }
}
