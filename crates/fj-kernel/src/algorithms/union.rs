use std::collections::BTreeSet;

use crate::objects::Solid;

use super::intersection::{self, CurveFaceIntersectionList};

/// Computes the shape that is the union of the two provided shapes
pub fn union(a: Solid, b: Solid) -> Solid {
    // TASK: Implement algorithm from "Boundary Representation Modelling
    //       Techniques", section 6.1.1 (pages 127 ff.).

    let mut faces = BTreeSet::new();

    for face_a in a.faces() {
        for face_b in b.faces() {
            let surface_a = face_a.surface();
            let surface_b = face_b.surface();

            let intersection =
                intersection::surface_surface(&surface_a, &surface_b);

            let (curve_a, curve_b, curve) = match intersection {
                Some(intersection) => intersection,
                None => {
                    // TASK: Implement.
                    continue;
                }
            };

            let intersections_a =
                CurveFaceIntersectionList::compute(&curve_a, face_a);
            let intersections_b =
                CurveFaceIntersectionList::compute(&curve_b, face_b);

            match (intersections_a.is_empty(), intersections_b.is_empty()) {
                (false, true) => {
                    faces.insert(face_a.clone());
                }
                (true, false) => {
                    faces.insert(face_b.clone());
                }
                (true, true) => {
                    faces.insert(face_a.clone());
                    faces.insert(face_b.clone());
                }
                _ => {
                    // TASK: Implement.
                    todo!()
                }
            }

            // TASK: Implement.
            let _ = curve;
        }
    }

    Solid::from_faces(faces)
}

#[cfg(test)]
mod tests {
    use crate::{
        algorithms::{union, TransformObject},
        objects::Solid,
    };

    #[test]
    fn distinct() {
        let a = Solid::cube_from_edge_length(1.).translate([-1., -1., -1.]);
        let b = Solid::cube_from_edge_length(1.).translate([1., 1., 1.]);

        let mut all_faces = Vec::new();
        all_faces.extend(a.faces().cloned());
        all_faces.extend(b.faces().cloned());

        let union = union(a, b);

        assert_eq!(union, Solid::from_faces(all_faces));
    }

    #[test]
    fn a_contains_b() {
        let a = Solid::cube_from_edge_length(2.);
        let b = Solid::cube_from_edge_length(1.);

        let union = union(a.clone(), b);

        assert_eq!(union, a);
    }

    #[test]
    fn b_contains_a() {
        let a = Solid::cube_from_edge_length(1.);
        let b = Solid::cube_from_edge_length(2.);

        let union = union(a, b.clone());

        assert_eq!(union, b);
    }

    // TASK: intersecting, broken edges in a
    // TASK: intersection, broken edges in b
}
