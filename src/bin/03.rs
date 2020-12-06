#[derive(PartialEq, Debug)]
enum Field {
    Empty,
    Tree
}

fn parse(line: &str) -> Vec<Field> {
    line.chars().map(|c| match c { '#' => Field::Tree, _ => Field::Empty }).collect::<Vec<Field>>()
}

fn count_trees(map:&Vec<Vec<Field>>, slope:(u32,u32)) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    let w = map[0].len();
    while y < map.len() {
        if map[y][x] == Field::Tree {
            count += 1
        }
        x = (x + slope.0 as usize) % w;
        y = y + slope.1 as usize;
    }
    count
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let input = advent::get_input(3).await?;
    let map = input.lines().map(parse).collect::<Vec<Vec<Field>>>();
    let count = count_trees(&map, (3,1));
    println!("{}", count);
    let answer:u32 = [(1,1),(3,1),(5,1),(7,1),(1,2)].iter().map(|s| count_trees(&map, *s)).product();
    println!("{}", answer);
    Ok(())
}
