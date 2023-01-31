use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let file_str = fs::read_to_string("./input.txt")?;
    let newline_delim = if file_str.find("\r\n") == None {
        "\n"
    } else {
        "\r\n"
    };

    let mut calorie_sums_vec = file_str
        .trim()
        .split(newline_delim.repeat(2).as_str())
        .map(|s| {
            s.split(newline_delim)
                .map(|s| s.parse::<u32>().unwrap_or_default())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    calorie_sums_vec.sort_by(|a, b| b.cmp(a));
    let top_three_sum: u32 = calorie_sums_vec.into_iter().take(3).sum();

    println!("Top three sum: {top_three_sum}");
    Ok(())
}
