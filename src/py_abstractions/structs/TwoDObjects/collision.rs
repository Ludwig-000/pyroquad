// TwoD collision is wayyyyy simpler than 3d.
// we simply implement some helper functions here.
//

use crate::py_abstractions::structs::{GLAM::Vec2::Vec2, TwoDObjects::{Circle::Circle, Rectangle::Rectangle}};
use macroquad::prelude as mq;

use pyo3::prelude::*;



#[derive(FromPyObject, IntoPyObject)]
pub enum Shape<'py> {
    Rect(Bound<'py, Rectangle>),
    Circ(Bound<'py, Circle>),
}



use rapier3d::parry::simba::scalar::SupersetOf;


pub fn collides_with_rec_rec(lhs: &Rectangle, rhs: &Rectangle) -> bool {


    let diff_x = rhs.position.x - lhs.position.x;
    let diff_y = rhs.position.y - lhs.position.y;

    // worst-case quick abort
    {   
        let lhs_radius = (lhs.scale.x * lhs.scale.x + lhs.scale.y * lhs.scale.y).sqrt() / 2.0;
        let rhs_radius = (rhs.scale.x * rhs.scale.x + rhs.scale.y * rhs.scale.y).sqrt() / 2.0;

        let dist_sq = diff_x * diff_x + diff_y * diff_y;
        let sum_radii = lhs_radius + rhs_radius;

        if dist_sq > sum_radii * sum_radii {
            return false;
        }
    }
    let axes = [
        (lhs.rotation.cos(), lhs.rotation.sin()),
        (-lhs.rotation.sin(), lhs.rotation.cos()),
        (rhs.rotation.cos(), rhs.rotation.sin()),
        (-rhs.rotation.sin(), rhs.rotation.cos()),
    ];

    for (ax, ay) in axes.iter() {
        let dist = (diff_x * ax + diff_y * ay).abs();

        let lhs_extents = (lhs.scale.x / 2.0) * (axes[0].0 * ax + axes[0].1 * ay).abs() +
                          (lhs.scale.y / 2.0) * (axes[1].0 * ax + axes[1].1 * ay).abs();

        let rhs_extents = (rhs.scale.x / 2.0) * (axes[2].0 * ax + axes[2].1 * ay).abs() +
                          (rhs.scale.y / 2.0) * (axes[3].0 * ax + axes[3].1 * ay).abs();

        if dist > (lhs_extents + rhs_extents) {
            return false;
        }
    }
    true
}


pub fn collides_with_rec_circ(lhs: &Rectangle, rhs: &Circle) -> bool {
    let diff_x = rhs.position.x - lhs.position.x;
    let diff_y = rhs.position.y - lhs.position.y;

    // worst-case quick abort
    {   
        let lhs_radius = (lhs.scale.x * lhs.scale.x + lhs.scale.y * lhs.scale.y).sqrt() / 2.0;

        let dist_sq = diff_x * diff_x + diff_y * diff_y;
        let sum_radii = lhs_radius + rhs.radius;

        if dist_sq > sum_radii * sum_radii {
            return false;
        }
    }

    let cos_r = lhs.rotation.cos();
    let sin_r = lhs.rotation.sin();
    
    let local_circ_x = diff_x * cos_r + diff_y * sin_r;
    let local_circ_y = -diff_x * sin_r + diff_y * cos_r;

    let half_w = lhs.scale.x / 2.0;
    let half_h = lhs.scale.y / 2.0;

    let closest_x = local_circ_x.clamp(-half_w, half_w);
    let closest_y = local_circ_y.clamp(-half_h, half_h);

    let dist_x = local_circ_x - closest_x;
    let dist_y = local_circ_y - closest_y;
    
    let dist_sq = dist_x * dist_x + dist_y * dist_y;

    dist_sq <= (rhs.radius * rhs.radius)
}


pub fn collides_with_circl_circ(lhs: &Circle, rhs: &Circle) -> bool{
    
    let dist_sq=  lhs.position.distance_squared(rhs.position);
    let combined_radius = lhs.radius + rhs.radius;

    dist_sq <= (combined_radius * combined_radius)
}