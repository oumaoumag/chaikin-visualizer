use macroquad::math::Vec2;

/// Applies one iteration of Chaikin's algorithm to a set of points
/// Returns a new set of points with corners "cut"
pub fn chaikin_iteration(points: &[Vec2], ratio: f32) -> Vec<Vec2> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    let mut new_points = Vec::new();

    for i in 0..points.len() {
        let p0 = points[i];
        let p1 = points[(i + 1) % points.len()];

        let q = p0 * (1.0 - ratio) + p1 * ratio;
        let r = p0 * ratio + p1 * (1.0 - ratio);

        new_points.push(q);
        new_points.push(r);
    }

    new_points
}

/// Applies Chaikin's algorithm for a specified number of iterations
pub fn apply_chaikin(points: &[Vec2], iterations: usize, ratio: f32) -> Vec<Vec<Vec2>> {
    let mut results = Vec::new();
    results.push(points.to_vec());

    let mut current_points = points.to_vec();

    for _ in 0..iterations {
        current_points = chaikin_iteration(&current_points, ratio);
        results.push(current_points.clone());
    }

    results
}
