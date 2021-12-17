struct TargetArea { xmin: i32, xmax: i32, ymin: i32, ymax: i32 }
struct Probe { x: i32, y: i32, xvel: i32, yvel: i32 }

fn step(probe: &mut Probe) {
    probe.x += probe.xvel;
    probe.y += probe.yvel;
    probe.yvel -= 1;
    if probe.xvel > 0 { probe.xvel -= 1 } else if probe.xvel < 0 { probe.xvel += 1 };
}

fn probe_will_hit(probe: &mut Probe, target: &TargetArea) -> Option<i32> {
    let mut y_peak = probe.y;

    // TODO: Improve how this works
    for _ in 0..1000 {
        step(probe);
        if probe.y > y_peak { y_peak = probe.y };

        if probe.x >= target.xmin && probe.x <= target.xmax && probe.y >= target.ymin && probe.y <= target.ymax {
            return Some(y_peak);
        }
    }

    None
}

fn part_one(target: &TargetArea) -> i32 {
    let mut x = 0;
    while (0..=x).sum::<i32>() < target.xmin {
        x += 1;
    }

    (0..1000)
        .filter_map(|y| probe_will_hit(&mut Probe { x: 0, y: 0, xvel: x, yvel: y }, target))
        .max()
        .unwrap()
}

fn part_two(target: &TargetArea) -> i32 {
    let mut hits = 0;

    for x in 0..=target.xmax {
        for y in target.ymin..1000 {
            if let Some(_) = probe_will_hit(&mut Probe { x: 0, y: 0, xvel: x, yvel: y }, target) {
                hits += 1;
            }
        }
    }

    hits
}

fn main() {
    let target = TargetArea { xmin: 169, xmax: 206, ymin: -108, ymax: -68 };
    println!("Day 17 Part 1: {}", part_one(&target));
    println!("Day 17 Part 2: {}", part_two(&target));
}

#[test]
fn test_part_one() {
    let target = TargetArea { xmin: 20, xmax: 30, ymin: -10, ymax: -5 };
    let mut probe_one = Probe { x: 0, y: 0, xvel: 6, yvel: 3 };
    assert!(probe_will_hit(&mut probe_one, &target).is_some());

    let mut probe_two = Probe { x: 0, y: 0, xvel: 17, yvel: -4 };
    assert_eq!(probe_will_hit(&mut probe_two, &target), None);

    let mut probe_three = Probe { x: 0, y: 0, xvel: 6, yvel: 9 };
    assert_eq!(probe_will_hit(&mut probe_three, &target), Some(45));
    assert_eq!(part_one(&target), 45);
}

#[test]
fn test_part_two() {
    let target = TargetArea { xmin: 20, xmax: 30, ymin: -10, ymax: -5 };
    assert_eq!(part_two(&target), 112);
}
