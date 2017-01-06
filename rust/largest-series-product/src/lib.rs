use std::collections::VecDeque;
use std::error::Error;

struct ProductKeeper {
    pub result : Result<u32, String>,
}

impl ProductKeeper {
    pub fn next(self, digit: Result<u32, std::num::ParseIntError>,
        current_series: &mut VecDeque<u32>, series_length: usize) -> ProductKeeper
    {
        match self.result {
            Ok(current_product) => match digit {
                Ok(digit) => { ProductKeeper { result: Ok(
                    Self::update_series(current_product, digit, current_series, series_length)
                ) } },
                Err(e) => ProductKeeper { result: Err(String::from("Parse Error: ") + e.description()) }
            },
            Err(parse_error) => ProductKeeper { result: Err(parse_error) }
        }
    }

    fn update_series(current_product: u32, digit: u32,
        current_series: &mut VecDeque<u32>, series_length: usize) -> u32
    {
        current_series.push_back(digit);
        if current_series.len() > series_length { current_series.pop_front(); }

        if current_series.len() == series_length {
            std::cmp::max(current_series.iter().product(), current_product)
        } else {
            current_product
        }
    }
}

pub fn lsp(series: &str, series_length: u32) -> Result<u32, String> {
    let slen = series_length as usize;
    if slen > series.len() {
        return Err(String::from("series_length cannot be longer than series."));
    } else if series.len() == 0 {
        return Ok(1);
    }

    let mut mem = VecDeque::new();
    let product_keeper = ProductKeeper { result: Ok(0) };

    series.chars()
        .map(|c| u32::from_str_radix(&c.to_string(), 10))
        .fold(product_keeper, |keeper, digit_result| keeper.next(digit_result, &mut mem, slen))
        .result
}
