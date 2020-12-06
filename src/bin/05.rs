struct BoardingPass {
    row: u32,
    col: u32,
    id: u32,
}

fn parse(pass: &str) -> BoardingPass {
    let mut cl = 0;
    let mut cu = 127;
    for c in pass[0..7].chars() {
        if c == 'F' {
            cu = cl + (cu-cl)/2
        } else {
            cl = cl + (cu-cl)/2 + 1
        }
    }
    let mut rl = 0;
    let mut ru = 7;
    for c in pass[7..10].chars() {
        if c == 'L' {
            ru = rl + (ru-rl)/2
        } else {
            rl = rl + (ru-rl)/2 + 1
        }
    }
    assert_eq!(cu, cl);
    assert_eq!(ru, rl);
    let id = cu * 8 + ru;
    BoardingPass {
        row: ru,
        col: cu,
        id
    }
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let mut ids = advent::get_input(5).await?.lines().map(parse).map(|p| p.id).collect::<Vec<_>>();
    ids.sort();
    println!("{}", ids.last().unwrap());
    let a = ids.iter();
    let b = a.clone().skip(1);
    let (before, after) = a.zip(b).find(|(&a,&b)| {a + 1 != b}).unwrap();
    println!("{}", before + 1);
    Ok(())
}
