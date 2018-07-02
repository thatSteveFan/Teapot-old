 use util::*;
 use util;

#[derive(Debug)]
pub struct Position
{
    pub vec: [f32; 3], 
}

#[derive(Debug)]
pub struct Direction
{
    pub forward: [f32; 3],
    pub up: [f32; 3]
}

#[derive(Debug)]
pub struct Camera
{
    pub pos: Position,
    pub dir: Direction
}

pub trait TripleRotateable
{
    fn rotate_x(&mut self, rad_x: f32);
    fn rotate_y(&mut self, rad_y: f32);
    fn rotate_z(&mut self, rad_z: f32);
}

impl TripleRotateable for Direction
{
    fn rotate_x(&mut self, rad: f32)
    {
        let rotate_matrix = rotate_axis_matrix(cross_product(self.up, self.forward), rad);
        self.forward = rotate_vec(rotate_matrix, self.forward);
        self.up = rotate_vec(rotate_matrix, self.up);
    }
    fn rotate_y(&mut self, rad: f32)
    {
        let rotate_matrix = rotate_axis_matrix(self.up, rad);
        self.forward = rotate_vec(rotate_matrix, self.forward);
        self.up = rotate_vec(rotate_matrix, self.up);
    }
    fn rotate_z(&mut self, rad: f32)
    {
        let rotate_matrix = rotate_axis_matrix(self.forward, rad);
        self.forward = rotate_vec(rotate_matrix, self.forward);
        self.up = rotate_vec(rotate_matrix, self.up);
    }
}

impl Direction
{
    pub fn normalize(&mut self)
    {
        util::normalize(&mut self.up);
        util::normalize(&mut self.forward);
        
    }
}

