use std::fs;

fn read_input(path: &str) -> (Decoder, Image) {
    let file = fs::read_to_string(path)
        .expect("File path must be valid");

    let mut lines = file.lines();

    let mut decoder: Decoder = [false; 512];
    lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .for_each(|(i, c)| decoder[i] = c == '#');

    lines.next().unwrap();
    let mut image = Image { marked: Vec::new(), background_dark: true };

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' { image.marked.push((x as i32, y as i32)); }
        }
    }

    (decoder, image)
}

struct Image {
    marked: Vec<(i32, i32)>,
    background_dark: bool
}

type Decoder = [bool; 512];

fn process(image: Image, decoder: &Decoder) -> Image {
    let lower_x_bound = image.marked.iter().min_by(|(x1, _), (x2, _)| x1.cmp(x2)).unwrap().0;
    let upper_x_bound = image.marked.iter().max_by(|(x1, _), (x2, _)| x1.cmp(x2)).unwrap().0;
    let lower_y_bound = image.marked.iter().min_by(|(_, y1), (_, y2)| y1.cmp(y2)).unwrap().1;
    let upper_y_bound = image.marked.iter().max_by(|(_, y1), (_, y2)| y1.cmp(y2)).unwrap().1;
    let flipped = decoder[0];

    let mut new_image = Image {
        marked: Vec::new(),
        background_dark: if flipped { !image.background_dark } else { image.background_dark }
    };

    for y in (lower_y_bound - 2)..=(upper_y_bound + 2) {
        for x in (lower_x_bound - 2)..=(upper_x_bound + 2) {
            let to_check = [
                (y - 1, x - 1), (y - 1, x), (y - 1, x + 1),
                (y    , x - 1), (y    , x), (y    , x + 1),
                (y + 1, x - 1), (y + 1, x), (y + 1, x + 1)
            ];

            let index = to_check
                .iter()
                .map(|(y, x)| {
                    let marked = image.marked.contains(&(*x, *y));
                    if image.background_dark { marked } else { !marked }
                })
                .fold(0, |acc, found| (acc * 2) + (if found { 1 } else { 0 }));

            let mut result = decoder[index];
            if !new_image.background_dark { result = !result; }
            if result { new_image.marked.push((x, y)); }
        }
    }

    new_image
}

fn part_one(image: Image, decoder: &Decoder) -> usize {
    let processed_once = process(image, decoder);
    let processed_twice = process(processed_once, decoder);
    processed_twice.marked.len()
}

fn main() {
    let (decoder, image) = read_input("input");
    println!("Day 20 Part 1: {:?}", part_one(image, &decoder));
}

// < 5495

#[test]
fn test_part_one() {
    let (decoder, image) = read_input("test");
    assert_eq!(part_one(image, &decoder), 35);
}
