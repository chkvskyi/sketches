use nannou::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct TreePoint {
    x: f32,
    y: f32,
    id: u32,
}

impl TreePoint {
    pub fn new(x: f32, y: f32, id: u32) -> Self {
        TreePoint { x, y, id }
    }
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

#[derive(Debug)]
pub struct QuadTree {
    capacity: usize,
    items: Vec<TreePoint>,
    boundary: Boundary,
    ne: Option<Box<QuadTree>>,
    nw: Option<Box<QuadTree>>,
    se: Option<Box<QuadTree>>,
    sw: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn new(boundary: Boundary, capacity: usize) -> QuadTree {
        QuadTree {
            capacity,
            boundary,
            items: Vec::<TreePoint>::new(),
            ne: None,
            nw: None,
            se: None,
            sw: None,
        }
    }

    pub fn insert(&mut self, item: TreePoint) {
        if self.boundary.contains(vec2(item.x(), item.y())) {
            if self.items.len() < self.capacity && !self.is_divided() {
                self.items.push(item);
            } else {
                if !self.is_divided() {
                    self.subdivide();
                }

                if let Some(ref mut q) = self.ne {
                    q.insert(item);
                }

                if let Some(ref mut q) = self.nw {
                    q.insert(item);
                }

                if let Some(ref mut q) = self.se {
                    q.insert(item);
                }

                if let Some(ref mut q) = self.sw {
                    q.insert(item);
                }
            }
        }
    }

    fn subdivide(&mut self) {
        let w = self.boundary.w / 2.;
        let h = self.boundary.h / 2.;
        let hw = w / 2.;
        let hh = h / 2.;

        let ne = QuadTree::new(
            Boundary {
                w,
                h,
                x: self.boundary.x + hw,
                y: self.boundary.y + hh,
            },
            self.capacity,
        );

        let nw = QuadTree::new(
            Boundary {
                w,
                h,
                x: self.boundary.x - hw,
                y: self.boundary.y + hh,
            },
            self.capacity,
        );

        let sw = QuadTree::new(
            Boundary {
                w,
                h,
                x: self.boundary.x - hw,
                y: self.boundary.y - hh,
            },
            self.capacity,
        );

        let se = QuadTree::new(
            Boundary {
                w,
                h,
                x: self.boundary.x + hw,
                y: self.boundary.y - hh,
            },
            self.capacity,
        );

        self.ne = Some(Box::new(ne));
        self.nw = Some(Box::new(nw));
        self.sw = Some(Box::new(sw));
        self.se = Some(Box::new(se));
    }

    fn is_divided(&self) -> bool {
        match &self.nw {
            Some(_) => true,
            _ => false,
        }
    }

    pub fn render(&self, draw: &Draw) {
        let arr = [
            self.boundary.top_left(),
            self.boundary.top_right(),
            self.boundary.bottom_right(),
            self.boundary.bottom_left(),
        ];
        let points = arr.iter().map(|&p| pt2(p.x, p.y));
        draw.polyline().weight(1.).points(points).color(WHITE);

        if let Some(ref q) = self.ne {
            q.render(draw);
        }

        if let Some(ref q) = self.nw {
            q.render(draw);
        }

        if let Some(ref q) = self.se {
            q.render(draw);
        }

        if let Some(ref q) = self.sw {
            q.render(draw);
        }
    }

    pub fn query(&self, b: &Boundary, found: &mut Vec<u32>) {
        if self.boundary.overlaps(&b) {
            for i in &self.items {
                if b.contains(vec2(i.x(), i.y())) {
                    found.push(i.id);
                }
            }

            if let Some(ref q) = self.ne {
                q.query(b, found);
            }

            if let Some(ref q) = self.nw {
                q.query(b, found);
            }

            if let Some(ref q) = self.se {
                q.query(b, found);
            }

            if let Some(ref q) = self.sw {
                q.query(b, found);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Boundary {
    pub w: f32,
    pub h: f32,
    pub x: f32,
    pub y: f32,
}

impl Boundary {
    pub fn contains(&self, point: Vector2) -> bool {
        let top_right = self.top_right();
        let bottom_left = self.bottom_left();

        point.x >= bottom_left.x
            && point.y >= bottom_left.y
            && point.x < top_right.x
            && point.y < top_right.y
    }

    pub fn bottom_left(&self) -> Vector2 {
        vec2(self.x - self.w / 2., self.y - self.h / 2.)
    }

    pub fn top_right(&self) -> Vector2 {
        vec2(self.x + self.w / 2., self.y + self.h / 2.)
    }

    pub fn top_left(&self) -> Vector2 {
        vec2(self.x - self.w / 2., self.y + self.h / 2.)
    }

    pub fn bottom_right(&self) -> Vector2 {
        vec2(self.x + self.w / 2., self.y - self.h / 2.)
    }

    pub fn overlaps(&self, other: &Boundary) -> bool {
        self.contains(other.top_left())
            || self.contains(other.top_right())
            || self.contains(other.bottom_right())
            || self.contains(other.bottom_left())
            || other.contains(self.top_left())
    }
}
