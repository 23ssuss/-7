struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut occupied = 0;
    let mut grid = vec![vec![false; 100]; 100];

    for rectangle in xs {
        let x1 = rectangle.a.x.min(rectangle.b.x);
        let x2 = rectangle.a.x.max(rectangle.b.x);
        let y1 = rectangle.a.y.min(rectangle.b.y);
        let y2 = rectangle.a.y.max(rectangle.b.y);

        for x in x1..x2 {
            for y in y1..y2 {
                if !grid[y as usize][x as usize] {
                    grid[y as usize][x as usize] = true;
                    occupied += 1;
                }
            }
        }
    }
    
    occupied
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

fn main() {
    area_occupied_test();
    println!("All tests passed successfully!");
}

