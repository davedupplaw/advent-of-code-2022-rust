mod day01;
mod day02;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    day01::day01::part1();
    day01::day01::part2();

    day02::day02::part1();
    day02::day02::part2();

    return Ok(())
}