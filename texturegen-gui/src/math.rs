use {Mat, Vect};

type Vec4<T> = ::nalgebra::Vector4<T>;

pub const TAU: f32 = 2. * ::std::f32::consts::PI;

pub fn flip_y(v: Vect) -> Vect {
    Vect::new(v.x, -v.y)
}

pub fn from_window_to_screen((w, h): (u32, u32), pos: [i32; 2]) -> Vect {
    Vect::new(pos[0] as f32 / w as f32, pos[1] as f32 / h as f32)
}

pub fn from_screen_to_world(cam: Mat, pos: Vect) -> Vect {
    let pos = inverse_transform(cam, pos);
    pos * 2.
}

pub fn from_world_to_screen(cam: Mat, pos: Vect) -> Vect {
    let pos = transform(cam, pos);
    Vect::new(pos.x + 1., -(pos.y + 1.))
}

pub fn transform(matrix: Mat, vector: Vect) -> Vect {
    let vector = Vec4::new(vector.x, vector.y, 0., 1.);
    let vector = matrix * vector;
    Vect::new(vector.x, vector.y)
}

pub fn inverse_transform(matrix: Mat, vector: Vect) -> Vect {
    let vector = Vec4::new(vector.x, vector.y, 0., 1.);
    let vector = matrix.try_inverse().unwrap() * vector;
    Vect::new(vector.x, vector.y)
}

pub fn scale(x: f32, y: f32) -> Mat {
    matrix([
        [x, 0., 0., 0.],
        [0., y, 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn translation(x: f32, y: f32) -> Mat {
    matrix([
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [x, y, 0., 1.],
    ])
}

pub fn matrix(matrix: [[f32; 4]; 4]) -> Mat {
    Mat::from_fn(|x, y| matrix[x][y])
}
