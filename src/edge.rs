#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Edge {
    TOPLEFT,
    TOPRIGHT,
    BOTTOMRIGHT,
    BOTTOMLEFT,
    LEFT,
    TOP,
    RIGHT,
    BOTTOM,
    NONE,
}

pub fn in_edge(x: i32, y: i32, xmax: i32, ymax: i32, offset: i32) -> Edge {
    if x == 0 && y == 0 {
        return Edge::TOPLEFT;
    }
    if x == xmax && y == 0 {
        return Edge::TOPRIGHT;
    }
    if x == xmax && y == ymax {
        return Edge::BOTTOMRIGHT;
    }
    if x == 0 && y == ymax {
        return Edge::BOTTOMLEFT;
    }
    if x == 0 && y > offset && y < ymax - offset {
        return Edge::LEFT;
    }
    if y == 0 && x > offset && x < xmax - offset {
        return Edge::TOP;
    }
    if x == xmax && y > offset && y < ymax - offset {
        return Edge::RIGHT;
    }
    if y == ymax && x > offset && x < xmax - offset {
        return Edge::BOTTOM;
    }
    Edge::NONE
}
