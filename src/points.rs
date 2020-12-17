use itertools::iproduct;

pub trait Point: Sized {
    fn get_neighbours(&self) -> Vec<Self>;
    fn new(from: (i32, i32, i32, i32)) -> Self;
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Point for Point3 {
    fn new(from: (i32, i32, i32, i32)) -> Self {
        Self {
            x: from.0,
            y: from.1,
            z: from.2,
        }
    }

    fn get_neighbours(&self) -> Vec<Self> {
        let xs = (self.x - 1)..=(self.x + 1);
        let ys = (self.y - 1)..=(self.y + 1);
        let zs = (self.z - 1)..=(self.z + 1);
        iproduct!(xs, ys, zs)
            .map(|(x, y, z)| Self::new((x, y, z, 0)))
            .filter(|p| self != p)
            .collect()
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point for Point4 {
    fn new(from: (i32, i32, i32, i32)) -> Self {
        Self {
            x: from.0,
            y: from.1,
            z: from.2,
            w: from.3,
        }
    }

    fn get_neighbours(&self) -> Vec<Self> {
        let xs = (self.x - 1)..=(self.x + 1);
        let ys = (self.y - 1)..=(self.y + 1);
        let zs = (self.z - 1)..=(self.z + 1);
        let ws = (self.w - 1)..=(self.w + 1);
        iproduct!(xs, ys, zs, ws)
            .map(Self::new)
            .filter(|p| self != p)
            .collect()
    }
}
