/// Chaos game is a binary that will "play" the [chaos
/// game](https://en.wikipedia.org/wiki/Chaos_game) to create points for a fractal. The points are
/// printed to stdout and can be viewed using a plotting tool like gnuplot.
///
/// # Usage
/// ### Generating the fractal
/// The following will write the points of a sierpinski triangle to plots/sierpinski-triangle.txt
/// `chaos-game sierpinski-triangle > plots/sierpinski-triangle.txt`
///
/// ### Viewing the fractal with gnuplot
/// `plot 'plots/sierpinski-triangle.txt' with points`
///
/// ### Animation of the fractal with gnuplot
/// `do for [i=0;1000000] { plot 'plots/vicsek.txt' every ::0::i }`
use rand::seq::SliceRandom;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn midpoint(&self, other: &Self) -> Self {
        let x = (self.x + other.x) / 2.0;
        let y = (self.y + other.y) / 2.0;
        Self { x, y }
    }

    fn jump_towards(&self, other: &Self, distance: f64) -> Self {
        let x = (self.x * (1.0 - distance)) + (other.x * distance);
        let y = (self.y * (1.0 - distance)) + (other.y * distance);
        Self { x, y }
    }
}

trait Polygon {
    fn points(&self) -> Vec<Point>;

    /// Assumes that adjacent points are next to each other in the array that comes out of points
    fn medial_points(&self) -> Vec<Point> {
        let points = self.points();

        let mut new_points = Vec::with_capacity(points.len());
        for (i, point) in self.points().iter().enumerate() {
            let next_point_index = (i + 1) % points.len();
            let next_point = points[next_point_index];

            let medial_point = point.midpoint(&next_point);
            new_points.push(medial_point);
        }

        new_points
    }

    /// Prints the x y coordinates of the generated fractal to stdout separated by a space. This can be fed into
    /// gnuplot to see the resulting fractal. `jump_distance` is how far to jump towards the next
    /// vertex and `next_point` is a closure to determine the vertex.
    fn chaos_game<F>(&self, iterations: usize, jump_distance: f64, next_point: &mut F)
    where
        F: FnMut() -> Point,
    {
        let mut rng = rand::thread_rng();
        let mut current_point = *self
            .medial_points()
            .choose(&mut rng)
            .expect("Shouldn't be empty");

        for _ in 0..iterations {
            println!("{} {}", current_point.x, current_point.y);
            let reference_point = next_point();
            let new_point = current_point.jump_towards(&reference_point, jump_distance);
            current_point = new_point;
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }

    fn new_equilateral(length: f64) -> Self {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(length, 0.0);
        let c = Point::new(length / 2.0, length);
        Triangle::new(a, b, c)
    }
}

impl Polygon for Triangle {
    fn points(&self) -> Vec<Point> {
        vec![self.a, self.b, self.c]
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Quadrilateral {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
}

impl Quadrilateral {
    pub fn new(a: Point, b: Point, c: Point, d: Point) -> Self {
        Self { a, b, c, d }
    }

    fn square(length: f64) -> Self {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(length, 0.0);
        let c = Point::new(length, length);
        let d = Point::new(0.0, length);
        Self::new(a, b, c, d)
    }
}

impl Polygon for Quadrilateral {
    fn points(&self) -> Vec<Point> {
        vec![self.a, self.b, self.c, self.d]
    }
}

const ITERATIONS: usize = 1000000;

fn main() {
    match std::env::args().nth(1) {
        Some(cmd) if cmd == "sierpinski-triangle" => sierpinski_triangle(ITERATIONS),
        Some(cmd) if cmd == "square-one" => square_one(ITERATIONS),
        Some(cmd) if cmd == "square-two" => square_two(ITERATIONS),
        Some(cmd) if cmd == "vicsek" => vicsek_fractal(ITERATIONS),
        None => sierpinski_triangle(ITERATIONS),
        Some(unrecognized) => panic!("{} is not yet implemented", unrecognized),
    }
}

#[allow(unused)]
fn sierpinski_triangle(iterations: usize) {
    let jump_distance = 0.5;
    let triangle = Triangle::new_equilateral(100.0);
    let mut rng = rand::thread_rng();
    let points = triangle.points();
    triangle.chaos_game(iterations, jump_distance, &mut || {
        *points.choose(&mut rng).expect("Shouldn't be empty")
    });
}

#[allow(unused)]
fn square_one(iterations: usize) {
    let jump_distance = 0.5;
    let square = Quadrilateral::square(100.0);
    let points = square.points();

    let mut rng = rand::thread_rng();
    let mut prev_vertex = points.choose(&mut rng).expect("Shouldn't be empty");
    square.chaos_game(iterations, jump_distance, &mut || loop {
        let new_vertex = points.choose(&mut rng).expect("Shouldn't be empty");
        if (new_vertex.x != prev_vertex.x || new_vertex.y != prev_vertex.y) {
            prev_vertex = new_vertex;
            break *new_vertex;
        }
    });
}

#[allow(unused)]
fn square_two(iterations: usize) {
    let jump_distance = 0.5;
    let square = Quadrilateral::square(100.0);
    let points = square.points();

    let mut rng = rand::thread_rng();
    let mut prev_vertex = points.choose(&mut rng).expect("Shouldn't be empty");
    square.chaos_game(iterations, jump_distance, &mut || loop {
        let new_vertex = points.choose(&mut rng).expect("Shouldn't be empty");
        if (new_vertex.x == prev_vertex.x || new_vertex.y == prev_vertex.y) {
            prev_vertex = new_vertex;
            break *new_vertex;
        }
    });
}

#[allow(unused)]
fn vicsek_fractal(iterations: usize) {
    let jump_distance = 0.66666666667;
    let square = Quadrilateral::square(100.0);
    let mut points = square.points();
    let midpoint = points[0].midpoint(&points[2]);
    points.push(midpoint);

    let mut rng = rand::thread_rng();
    let mut prev_vertex = points.choose(&mut rng).expect("Shouldn't be empty");
    square.chaos_game(iterations, jump_distance, &mut || loop {
        let new_vertex = points.choose(&mut rng).expect("Shouldn't be empty");
        break *new_vertex;
    });
}
