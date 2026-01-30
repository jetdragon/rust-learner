// exercise11_result_operations.rs
pub fn map_result_option(opt: Option<i32>) -> Result<i32, String> {
    opt.map_or(Err("没有值".to_string()), |v| Ok(v * 2))
}

pub fn combine_results(
    a: Result<i32, String>,
    b: Result<i32, String>,
) -> Result<(i32, i32), String> {
    match (a, b) {
        (Ok(x), Ok(y)) => Ok((x, y)),
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

pub fn unwrap_or_default(result: Result<i32, String>) -> i32 {
    result.unwrap_or(0)
}

pub fn and_then_option(r: Result<i32, String>) -> Option<i32> {
    r.ok()
}
