use crate::Vector::Vector3;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Triangle{
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3,
}


impl Triangle {

    pub fn moller_trumbore_intersection (&self, origin: Vector3, direction: Vector3) -> Option<Vector3> {
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;

        let ray_cross_e2 = direction * e2;
        let det = e1.dot(&ray_cross_e2);

        if det > -f64::EPSILON && det < f64::EPSILON {
            return None; // This ray is parallel to this triangle.
        }

        let inv_det = 1.0 / det;
        let s = origin - self.a;
        let u = inv_det * s.dot(&ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = s*e1;
        let v = inv_det * direction.dot(&s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = inv_det * e2.dot(&s_cross_e1);

        if t > f64::EPSILON { // ray intersection
            let intersection_point = origin + direction * t;
            return Some(intersection_point);
        }
        else { // This means that there is a line intersection but not a ray intersection.
            return None;
        }
    }

    pub fn normal(&self) -> Vector3 {
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        edge1*edge2
    }

    pub fn unit_normal(&self) -> Vector3 {
        self.normal().unitise()
    }
}
