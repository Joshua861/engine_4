#[cfg(test)]
mod tests {
    use crate::collisions::ray::*;
    use crate::collisions::*;
    use bevy_math::Vec2;

    // Circle-Circle tests
    #[test]
    fn circle_circle_intersect() {
        let c1 = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let c2 = Circle {
            center: Vec2::new(8.0, 0.0),
            radius: 5.0,
        };
        assert!(c1.intersects_with(&c2));
    }

    #[test]
    fn circle_circle_touching() {
        let c1 = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let c2 = Circle {
            center: Vec2::new(10.0, 0.0),
            radius: 5.0,
        };
        assert!(c1.intersects_with(&c2));
    }

    #[test]
    fn circle_circle_no_intersect() {
        let c1 = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let c2 = Circle {
            center: Vec2::new(20.0, 0.0),
            radius: 5.0,
        };
        assert!(!c1.intersects_with(&c2));
    }

    #[test]
    fn circle_circle_contained() {
        let c1 = Circle {
            center: Vec2::ZERO,
            radius: 10.0,
        };
        let c2 = Circle {
            center: Vec2::new(1.0, 1.0),
            radius: 2.0,
        };
        assert!(c1.intersects_with(&c2));
    }

    // Circle-Square tests
    #[test]
    fn circle_square_intersect() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let square = Square {
            center: Vec2::new(7.0, 0.0),
            half_size: 5.0,
        };
        assert!(circle.intersects_with(&square));
    }

    #[test]
    fn circle_square_corner_intersect() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let square = Square {
            center: Vec2::new(7.0, 7.0),
            half_size: 5.0,
        };
        assert!(circle.intersects_with(&square));
    }

    #[test]
    fn circle_square_no_intersect() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let square = Square {
            center: Vec2::new(20.0, 0.0),
            half_size: 5.0,
        };
        assert!(!circle.intersects_with(&square));
    }

    #[test]
    fn circle_inside_square() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 2.0,
        };
        let square = Square {
            center: Vec2::ZERO,
            half_size: 10.0,
        };
        assert!(circle.intersects_with(&square));
    }

    // Square-Square tests
    #[test]
    fn square_square_intersect() {
        let s1 = Square {
            center: Vec2::ZERO,
            half_size: 5.0,
        };
        let s2 = Square {
            center: Vec2::new(7.0, 0.0),
            half_size: 5.0,
        };
        assert!(s1.intersects_with(&s2));
    }

    #[test]
    fn square_square_touching() {
        let s1 = Square {
            center: Vec2::ZERO,
            half_size: 5.0,
        };
        let s2 = Square {
            center: Vec2::new(10.0, 0.0),
            half_size: 5.0,
        };
        assert!(s1.intersects_with(&s2));
    }

    #[test]
    fn square_square_no_intersect() {
        let s1 = Square {
            center: Vec2::ZERO,
            half_size: 5.0,
        };
        let s2 = Square {
            center: Vec2::new(20.0, 0.0),
            half_size: 5.0,
        };
        assert!(!s1.intersects_with(&s2));
    }

    // Point tests
    #[test]
    fn point_in_circle() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let point = Point::new(Vec2::new(3.0, 0.0));
        assert!(circle.intersects_with(&point));
    }

    #[test]
    fn point_outside_circle() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let point = Point::new(Vec2::new(10.0, 0.0));
        assert!(!circle.intersects_with(&point));
    }

    #[test]
    fn point_in_square() {
        let square = Square {
            center: Vec2::ZERO,
            half_size: 5.0,
        };
        let point = Point::new(Vec2::new(3.0, 3.0));
        assert!(square.intersects_with(&point));
    }

    #[test]
    fn point_outside_square() {
        let square = Square {
            center: Vec2::ZERO,
            half_size: 5.0,
        };
        let point = Point::new(Vec2::new(10.0, 10.0));
        assert!(!square.intersects_with(&point));
    }

    // Polygon tests
    #[test]
    fn triangle_contains_point() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(10.0, 0.0),
                Vec2::new(5.0, 10.0),
            ],
        };
        assert!(polygon.contains_point(Vec2::new(5.0, 3.0)));
    }

    #[test]
    fn triangle_does_not_contain_point() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(10.0, 0.0),
                Vec2::new(5.0, 10.0),
            ],
        };
        assert!(!polygon.contains_point(Vec2::new(15.0, 15.0)));
    }

    #[test]
    fn polygon_circle_intersect() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(10.0, 0.0),
                Vec2::new(10.0, 10.0),
                Vec2::new(0.0, 10.0),
            ],
        };
        let circle = Circle {
            center: Vec2::new(5.0, 5.0),
            radius: 2.0,
        };
        assert!(polygon.intersects_with(&circle));
    }

    #[test]
    fn polygon_circle_edge_intersect() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(10.0, 0.0),
                Vec2::new(10.0, 10.0),
                Vec2::new(0.0, 10.0),
            ],
        };
        let circle = Circle {
            center: Vec2::new(12.0, 5.0),
            radius: 3.0,
        };
        assert!(polygon.intersects_with(&circle));
    }

    #[test]
    fn polygon_polygon_intersect() {
        let poly1 = Polygon {
            vertices: vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(10.0, 0.0),
                Vec2::new(10.0, 10.0),
                Vec2::new(0.0, 10.0),
            ],
        };
        let poly2 = Polygon {
            vertices: vec![
                Vec2::new(5.0, 5.0),
                Vec2::new(15.0, 5.0),
                Vec2::new(15.0, 15.0),
                Vec2::new(5.0, 15.0),
            ],
        };
        assert!(poly1.intersects_with(&poly2));
    }

    // AABB tests
    #[test]
    fn aabb_intersect() {
        let aabb1 = AABB2D::new(Vec2::ZERO, Vec2::new(10.0, 10.0));
        let aabb2 = AABB2D::new(Vec2::new(5.0, 5.0), Vec2::new(15.0, 15.0));
        assert!(aabb1.intersects(&aabb2));
    }

    #[test]
    fn aabb_no_intersect() {
        let aabb1 = AABB2D::new(Vec2::ZERO, Vec2::new(10.0, 10.0));
        let aabb2 = AABB2D::new(Vec2::new(20.0, 20.0), Vec2::new(30.0, 30.0));
        assert!(!aabb1.intersects(&aabb2));
    }

    #[test]
    fn aabb_from_center_size() {
        let aabb = AABB2D::from_center_size(Vec2::new(5.0, 5.0), Vec2::new(10.0, 10.0));
        assert_eq!(aabb.min, Vec2::ZERO);
        assert_eq!(aabb.max, Vec2::new(10.0, 10.0));
    }

    #[test]
    fn aabb_expand() {
        let aabb = AABB2D::new(Vec2::new(5.0, 5.0), Vec2::new(10.0, 10.0));
        let expanded = aabb.expand(2.0);
        assert_eq!(expanded.min, Vec2::new(3.0, 3.0));
        assert_eq!(expanded.max, Vec2::new(12.0, 12.0));
    }

    // HasBounds tests
    #[test]
    fn circle_bounds() {
        let circle = Circle {
            center: Vec2::new(5.0, 5.0),
            radius: 3.0,
        };
        let bounds = circle.bounds();
        assert_eq!(bounds.min, Vec2::new(2.0, 2.0));
        assert_eq!(bounds.max, Vec2::new(8.0, 8.0));
    }

    #[test]
    fn square_bounds() {
        let square = Square {
            center: Vec2::new(5.0, 5.0),
            half_size: 3.0,
        };
        let bounds = square.bounds();
        assert_eq!(bounds.min, Vec2::new(2.0, 2.0));
        assert_eq!(bounds.max, Vec2::new(8.0, 8.0));
    }

    #[test]
    fn polygon_bounds() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(1.0, 2.0),
                Vec2::new(5.0, 1.0),
                Vec2::new(7.0, 6.0),
                Vec2::new(3.0, 8.0),
            ],
        };
        let bounds = polygon.bounds();
        assert_eq!(bounds.min, Vec2::new(1.0, 1.0));
        assert_eq!(bounds.max, Vec2::new(7.0, 8.0));
    }

    // Symmetry tests
    #[test]
    fn intersection_symmetry() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let square = Square {
            center: Vec2::new(7.0, 0.0),
            half_size: 5.0,
        };
        assert_eq!(
            circle.intersects_with(&square),
            square.intersects_with(&circle)
        );
    }

    #[test]
    fn collision_then_raycast() {
        let circle1 = Circle {
            center: Vec2::ZERO,
            radius: 5.0,
        };
        let circle2 = Circle {
            center: Vec2::new(8.0, 0.0),
            radius: 5.0,
        };

        // They should intersect
        assert!(circle1.intersects_with(&circle2));

        // Ray from circle1 to circle2 should hit
        let ray = Ray::from_points(circle1.center, circle2.center);
        assert!(circle2.raycast(&ray).is_some());
    }

    #[test]
    fn aabb_contains_shapes() {
        let circle = Circle {
            center: Vec2::new(5.0, 5.0),
            radius: 2.0,
        };
        let square = Square {
            center: Vec2::new(5.0, 5.0),
            half_size: 3.0,
        };

        let circle_bounds = circle.bounds();
        let square_bounds = square.bounds();

        // Square should contain circle
        assert!(square_bounds.intersects(&circle_bounds));
    }

    #[test]
    fn point_in_all_overlapping_shapes() {
        let point = Point::new(Vec2::new(5.0, 5.0));

        let circle = Circle {
            center: Vec2::new(5.0, 5.0),
            radius: 3.0,
        };
        let square = Square {
            center: Vec2::new(5.0, 5.0),
            half_size: 4.0,
        };
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(3.0, 3.0),
                Vec2::new(7.0, 3.0),
                Vec2::new(7.0, 7.0),
                Vec2::new(3.0, 7.0),
            ],
        };

        assert!(circle.intersects_with(&point));
        assert!(square.intersects_with(&point));
        assert!(polygon.intersects_with(&point));
    }

    #[test]
    fn raycast_through_overlapping_shapes() {
        let circle = Circle {
            center: Vec2::new(10.0, 0.0),
            radius: 3.0,
        };
        let square = Square {
            center: Vec2::new(10.0, 0.0),
            half_size: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));
        let circle_hit = circle.raycast(&ray);
        let square_hit = square.raycast(&ray);
        assert!(circle_hit.is_some());
        assert!(square_hit.is_some());

        assert!(square_hit.unwrap().distance < circle_hit.unwrap().distance);
    }

    #[test]
    fn bounds_after_collision() {
        let mut shapes: Vec<Box<dyn HasBounds2D>> = vec![
            Box::new(Circle {
                center: Vec2::new(0.0, 0.0),
                radius: 5.0,
            }),
            Box::new(Square {
                center: Vec2::new(8.0, 0.0),
                half_size: 3.0,
            }),
        ];

        // Get all bounds
        let bounds: Vec<AABB2D> = shapes.iter().map(|s| s.bounds()).collect();

        // Check if any bounds intersect
        let mut intersections = 0;
        for i in 0..bounds.len() {
            for j in (i + 1)..bounds.len() {
                if bounds[i].intersects(&bounds[j]) {
                    intersections += 1;
                }
            }
        }

        assert!(intersections > 0);
    }

    #[test]
    fn complex_scene() {
        // Create a scene with multiple shapes
        let shapes = vec![
            Circle {
                center: Vec2::new(0.0, 0.0),
                radius: 5.0,
            },
            Circle {
                center: Vec2::new(20.0, 0.0),
                radius: 5.0,
            },
            Circle {
                center: Vec2::new(40.0, 0.0),
                radius: 5.0,
            },
        ];

        // Cast a ray through all of them
        let ray = Ray::new(Vec2::new(-10.0, 0.0), Vec2::new(1.0, 0.0));

        let hits: Vec<_> = shapes
            .iter()
            .filter_map(|shape| shape.raycast(&ray))
            .collect();

        assert_eq!(hits.len(), 3);

        // Check they're in order
        for i in 0..hits.len() - 1 {
            assert!(hits[i].distance < hits[i + 1].distance);
        }
    }

    #[test]
    fn spatial_partitioning_scenario() {
        // Simulate a grid-based collision system
        let grid_size = 10.0;
        let shapes = vec![
            (
                0,
                0,
                Circle {
                    center: Vec2::new(5.0, 5.0),
                    radius: 2.0,
                },
            ),
            (
                1,
                0,
                Circle {
                    center: Vec2::new(15.0, 5.0),
                    radius: 2.0,
                },
            ),
            (
                0,
                1,
                Circle {
                    center: Vec2::new(5.0, 15.0),
                    radius: 2.0,
                },
            ),
        ];

        let query_point = Point::new(Vec2::new(6.0, 6.0));

        // Only check shapes in the same grid cell (0, 0)
        let nearby_shapes: Vec<_> = shapes
            .iter()
            .filter(|(x, y, _)| *x == 0 && *y == 0)
            .collect();

        assert_eq!(nearby_shapes.len(), 1);
        assert!(nearby_shapes[0].2.intersects_with(&query_point));
    }

    #[test]
    fn raycast_returns_closest_hit() {
        let shapes = vec![
            Circle {
                center: Vec2::new(10.0, 0.0),
                radius: 2.0,
            },
            Circle {
                center: Vec2::new(20.0, 0.0),
                radius: 2.0,
            },
            Circle {
                center: Vec2::new(30.0, 0.0),
                radius: 2.0,
            },
        ];

        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        // Find closest hit manually
        let closest = shapes
            .iter()
            .filter_map(|shape| shape.raycast(&ray))
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        assert!(closest.is_some());
        let closest = closest.unwrap();

        // Should hit the first circle
        assert!((closest.distance - 8.0).abs() < 0.1);
    }

    #[test]
    fn collision_filtering_by_bounds() {
        let shapes = vec![
            Circle {
                center: Vec2::new(0.0, 0.0),
                radius: 5.0,
            },
            Circle {
                center: Vec2::new(100.0, 100.0),
                radius: 5.0,
            },
            Circle {
                center: Vec2::new(200.0, 200.0),
                radius: 5.0,
            },
        ];

        let query_bounds = AABB2D::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0));

        // Filter shapes by bounds first
        let nearby: Vec<_> = shapes
            .iter()
            .filter(|shape| shape.bounds().intersects(&query_bounds))
            .collect();

        assert_eq!(nearby.len(), 1);
    }
}

#[cfg(test)]
mod collision_performance_tests {
    use crate::collisions::ray::*;
    use crate::collisions::*;
    use bevy_math::Vec2;

    #[test]
    fn many_circle_collisions() {
        let circles: Vec<Circle> = (0..100)
            .map(|i| Circle {
                center: Vec2::new((i % 10) as f32 * 10.0, (i / 10) as f32 * 10.0),
                radius: 5.0,
            })
            .collect();

        let test_circle = Circle {
            center: Vec2::new(45.0, 45.0),
            radius: 8.0,
        };

        let mut collision_count = 0;
        for circle in &circles {
            if test_circle.intersects_with(circle) {
                collision_count += 1;
            }
        }

        assert!(collision_count > 0);
    }

    #[test]
    fn many_raycasts() {
        let shapes: Vec<Circle> = (0..50)
            .map(|i| Circle {
                center: Vec2::new((i as f32) * 20.0, 0.0),
                radius: 5.0,
            })
            .collect();

        let ray = Ray::new(Vec2::new(-10.0, 0.0), Vec2::new(1.0, 0.0));

        let mut hit_count = 0;
        for shape in &shapes {
            if shape.raycast(&ray).is_some() {
                hit_count += 1;
            }
        }

        assert_eq!(hit_count, shapes.len());
    }

    #[test]
    fn aabb_broadphase_effectiveness() {
        let shapes: Vec<Square> = (0..100)
            .map(|i| Square {
                center: Vec2::new((i % 10) as f32 * 100.0, (i / 10) as f32 * 100.0),
                half_size: 10.0,
            })
            .collect();

        let test_aabb = AABB2D::new(Vec2::new(0.0, 0.0), Vec2::new(50.0, 50.0));

        let mut potential_collisions = 0;
        for shape in &shapes {
            if test_aabb.intersects(&shape.bounds()) {
                potential_collisions += 1;
            }
        }

        // Should filter out most shapes
        assert!(potential_collisions < shapes.len() / 2);
    }
}

#[cfg(test)]
mod camera_tests {
    use crate::camera::*;
    use crate::prelude::*;
    use bevy_math::Vec2;

    fn create_test_camera() -> Camera2D {
        Camera2D::new(800, 600)
    }

    #[test]
    fn test_camera_initialization() {
        let camera = create_test_camera();
        assert_eq!(camera.translation, Vec2::ZERO);
        assert_eq!(camera.scale, 1.0);
        assert_eq!(camera.rotation, 0.0);
        assert_eq!(camera.window_size(), Vec2::new(800.0, 600.0));
    }

    #[test]
    fn test_screen_center() {
        let camera = create_test_camera();
        let center = camera.window_size() * 0.5;
        assert_eq!(center, Vec2::new(400.0, 300.0));
    }

    #[test]
    fn test_screen_to_world_identity() {
        let mut camera = create_test_camera();
        let screen_pos = Vec2::new(400.0, 300.0); // center
        let world_pos = camera.screen_to_world(screen_pos);
        assert_eq!(world_pos, Vec2::ZERO);
    }

    #[test]
    fn test_world_to_screen_identity() {
        let mut camera = create_test_camera();
        let world_pos = Vec2::ZERO;
        let screen_pos = camera.world_to_screen(world_pos);
        assert_eq!(screen_pos, Vec2::new(400.0, 300.0));
    }

    #[test]
    fn test_screen_world_roundtrip() {
        let mut camera = create_test_camera();
        let original = Vec2::new(100.0, 200.0);
        let world = camera.screen_to_world(original);
        let back = camera.world_to_screen(world);

        assert!((original.x - back.x).abs() < 0.001);
        assert!((original.y - back.y).abs() < 0.001);
    }

    #[test]
    fn test_translation() {
        let mut camera = create_test_camera();
        camera.translation = Vec2::new(100.0, 50.0);
        camera.mark_dirty();

        let world_pos = camera.screen_to_world(Vec2::new(400.0, 300.0));
        assert_eq!(world_pos, Vec2::new(100.0, 50.0));
    }

    #[test]
    fn test_scale_doubles() {
        let mut camera = create_test_camera();
        camera.scale = 2.0;
        camera.mark_dirty();

        let screen_pos = Vec2::new(500.0, 300.0); // 100px right of center
        let world_pos = camera.screen_to_world(screen_pos);
        assert_eq!(world_pos, Vec2::new(50.0, 0.0)); // Should be 50 units in world
    }

    #[test]
    fn test_scale_halves() {
        let mut camera = create_test_camera();
        camera.scale = 0.5;
        camera.mark_dirty();

        let screen_pos = Vec2::new(500.0, 300.0); // 100px right of center
        let world_pos = camera.screen_to_world(screen_pos);
        assert_eq!(world_pos, Vec2::new(200.0, 0.0)); // Should be 200 units in world
    }

    #[test]
    fn test_zoom_at_maintains_point() {
        let mut camera = create_test_camera();
        let screen_point = Vec2::new(500.0, 400.0);
        let world_before = camera.screen_to_world(screen_point);

        camera.zoom_at(screen_point, 2.0);

        let world_after = camera.screen_to_world(screen_point);

        assert!((world_before.x - world_after.x).abs() < 0.1);
        assert!((world_before.y - world_after.y).abs() < 0.1);
    }

    #[test]
    fn test_visible_bounds_no_transform() {
        let mut camera = create_test_camera();
        let (min, max) = camera.visible_bounds();

        assert_eq!(min, Vec2::new(-400.0, -300.0));
        assert_eq!(max, Vec2::new(400.0, 300.0));
    }

    #[test]
    fn test_visible_bounds_with_translation() {
        let mut camera = create_test_camera();
        camera.translation = Vec2::new(100.0, 50.0);
        camera.mark_dirty();

        let (min, max) = camera.visible_bounds();

        assert_eq!(min, Vec2::new(-300.0, -250.0));
        assert_eq!(max, Vec2::new(500.0, 350.0));
    }

    #[test]
    fn test_visible_bounds_with_scale() {
        let mut camera = create_test_camera();
        camera.scale = 2.0;
        camera.mark_dirty();

        let (min, max) = camera.visible_bounds();

        assert_eq!(min, Vec2::new(-200.0, -150.0));
        assert_eq!(max, Vec2::new(200.0, 150.0));
    }

    #[test]
    fn test_distance_conversions() {
        let camera = create_test_camera();
        let world_dist = 100.0;
        let screen_dist = camera.world_distance_to_screen(world_dist);
        assert_eq!(screen_dist, 100.0);

        let back = camera.screen_distance_to_world(screen_dist);
        assert_eq!(back, world_dist);
    }

    #[test]
    fn test_distance_conversions_scaled() {
        let mut camera = create_test_camera();
        camera.scale = 2.0;

        let world_dist = 100.0;
        let screen_dist = camera.world_distance_to_screen(world_dist);
        assert_eq!(screen_dist, 200.0);

        let back = camera.screen_distance_to_world(screen_dist);
        assert_eq!(back, world_dist);
    }

    #[test]
    fn test_update_sizes() {
        let mut camera = create_test_camera();
        camera.update_sizes(1024, 768);

        assert_eq!(camera.window_size(), Vec2::new(1024.0, 768.0));
    }

    #[test]
    fn test_multiple_transformations() {
        let mut camera = create_test_camera();
        camera.translation = Vec2::new(50.0, 25.0);
        camera.scale = 1.5;
        camera.mark_dirty();

        let screen_pos = Vec2::new(500.0, 400.0);
        let world_pos = camera.screen_to_world(screen_pos);
        let back = camera.world_to_screen(world_pos);

        assert!((screen_pos.x - back.x).abs() < 0.001);
        assert!((screen_pos.y - back.y).abs() < 0.001);
    }
}

#[cfg(test)]
mod raycast_tests {
    use crate::collisions::ray::*;
    use crate::collisions::*;
    use bevy_math::Vec2;

    // Basic ray tests
    #[test]
    fn test_ray_creation() {
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));
        assert_eq!(ray.origin, Vec2::ZERO);
        assert_eq!(ray.direction, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn test_ray_from_points() {
        let ray = Ray::from_points(Vec2::ZERO, Vec2::new(10.0, 0.0));
        assert_eq!(ray.origin, Vec2::ZERO);
        assert_eq!(ray.direction, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn test_ray_direction_normalized() {
        let ray = Ray::new(Vec2::ZERO, Vec2::new(3.0, 4.0));
        let length = ray.direction.length();
        assert!((length - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ray_point_at() {
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));
        let point = ray.point_at(5.0);
        assert_eq!(point, Vec2::new(5.0, 0.0));
    }

    // Circle raycast tests
    #[test]
    fn test_ray_hits_circle_center() {
        let circle = Circle {
            center: Vec2::new(10.0, 0.0),
            radius: 3.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = circle.raycast(&ray);
        assert!(hit.is_some());

        let hit = hit.unwrap();
        assert!((hit.distance - 7.0).abs() < 0.001);
        assert_eq!(hit.normal, Vec2::new(-1.0, 0.0));
    }

    #[test]
    fn test_ray_misses_circle() {
        let circle = Circle {
            center: Vec2::new(10.0, 10.0),
            radius: 2.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        assert!(circle.raycast(&ray).is_none());
    }

    #[test]
    fn test_ray_inside_circle() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 10.0,
        };
        let ray = Ray::new(Vec2::new(5.0, 0.0), Vec2::new(1.0, 0.0));

        let hit = circle.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_tangent_to_circle() {
        let circle = Circle {
            center: Vec2::new(0.0, 5.0),
            radius: 5.0,
        };
        let ray = Ray::new(Vec2::new(-10.0, 0.0), Vec2::new(1.0, 0.0));

        let hit = circle.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_behind_circle() {
        let circle = Circle {
            center: Vec2::new(-10.0, 0.0),
            radius: 3.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        assert!(circle.raycast(&ray).is_none());
    }

    // Square raycast tests
    #[test]
    fn test_ray_hits_square_front() {
        let square = Square {
            center: Vec2::new(10.0, 0.0),
            half_size: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = square.raycast(&ray);
        assert!(hit.is_some());

        let hit = hit.unwrap();
        assert!((hit.distance - 5.0).abs() < 0.001);
        assert_eq!(hit.normal, Vec2::new(-1.0, 0.0));
    }

    #[test]
    fn test_ray_hits_square_corner() {
        let square = Square {
            center: Vec2::new(10.0, 10.0),
            half_size: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 1.0).normalize());

        let hit = square.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_misses_square() {
        let square = Square {
            center: Vec2::new(10.0, 10.0),
            half_size: 2.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        assert!(square.raycast(&ray).is_none());
    }

    #[test]
    fn test_ray_inside_square() {
        let square = Square {
            center: Vec2::ZERO,
            half_size: 10.0,
        };
        let ray = Ray::new(Vec2::new(5.0, 0.0), Vec2::new(1.0, 0.0));

        let hit = square.raycast(&ray);
        assert!(hit.is_some());
    }

    // Polygon raycast tests
    #[test]
    fn test_ray_hits_triangle() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(10.0, -5.0),
                Vec2::new(10.0, 5.0),
                Vec2::new(15.0, 0.0),
            ],
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = polygon.raycast(&ray);
        assert!(hit.is_some());

        let hit = hit.unwrap();
        assert!((hit.distance - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_ray_misses_triangle() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(10.0, 10.0),
                Vec2::new(15.0, 10.0),
                Vec2::new(12.5, 15.0),
            ],
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        assert!(polygon.raycast(&ray).is_none());
    }

    #[test]
    fn test_ray_hits_square_polygon() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(10.0, -5.0),
                Vec2::new(15.0, -5.0),
                Vec2::new(15.0, 5.0),
                Vec2::new(10.0, 5.0),
            ],
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = polygon.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_hits_closest_edge() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(10.0, -5.0),
                Vec2::new(15.0, -5.0),
                Vec2::new(15.0, 5.0),
                Vec2::new(10.0, 5.0),
            ],
        };
        let ray = Ray::new(Vec2::new(-5.0, 0.0), Vec2::new(1.0, 0.0));

        let hit = polygon.raycast(&ray);
        assert!(hit.is_some());

        let hit = hit.unwrap();
        // Should hit the left edge at x=10
        assert!((hit.point.x - 10.0).abs() < 0.001);
    }

    // Point raycast tests
    #[test]
    fn test_ray_hits_point() {
        let point = Point::new(Vec2::new(10.0, 0.0));
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = point.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_misses_point() {
        let point = Point::new(Vec2::new(10.0, 5.0));
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        assert!(point.raycast(&ray).is_none());
    }

    // Max distance tests
    #[test]
    fn test_raycast_max_distance_hit() {
        let circle = Circle {
            center: Vec2::new(10.0, 0.0),
            radius: 3.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        assert!(circle.raycast_max(&ray, 20.0).is_some());
    }

    #[test]
    fn test_raycast_max_distance_miss() {
        let circle = Circle {
            center: Vec2::new(100.0, 0.0),
            radius: 3.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        assert!(circle.raycast_max(&ray, 50.0).is_none());
    }

    // Multiple hits tests
    #[test]
    fn test_ray_through_multiple_circles() {
        let circle1 = Circle {
            center: Vec2::new(10.0, 0.0),
            radius: 3.0,
        };
        let circle2 = Circle {
            center: Vec2::new(20.0, 0.0),
            radius: 3.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit1 = circle1.raycast(&ray);
        let hit2 = circle2.raycast(&ray);

        assert!(hit1.is_some());
        assert!(hit2.is_some());

        // First circle should be closer
        assert!(hit1.unwrap().distance < hit2.unwrap().distance);
    }

    // Diagonal ray tests
    #[test]
    fn test_diagonal_ray_hits_circle() {
        let circle = Circle {
            center: Vec2::new(10.0, 10.0),
            radius: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 1.0).normalize());

        let hit = circle.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_diagonal_ray_hits_square() {
        let square = Square {
            center: Vec2::new(10.0, 10.0),
            half_size: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 1.0).normalize());

        let hit = square.raycast(&ray);
        assert!(hit.is_some());
    }

    // Normal vector tests
    #[test]
    fn test_circle_hit_normal_pointing_out() {
        let circle = Circle {
            center: Vec2::new(10.0, 0.0),
            radius: 3.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = circle.raycast(&ray).unwrap();
        // Normal should point back towards ray origin
        assert!(hit.normal.dot(ray.direction) < 0.0);
    }

    #[test]
    fn test_square_hit_normal_perpendicular() {
        let square = Square {
            center: Vec2::new(10.0, 0.0),
            half_size: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = square.raycast(&ray).unwrap();
        // Normal should be perpendicular to the hit surface
        assert_eq!(hit.normal.length(), 1.0);
    }

    // Edge cases
    #[test]
    fn test_ray_origin_on_circle() {
        let circle = Circle {
            center: Vec2::ZERO,
            radius: 10.0,
        };
        let ray = Ray::new(Vec2::new(10.0, 0.0), Vec2::new(1.0, 0.0));

        // Ray starting on circle surface
        let hit = circle.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_origin_on_square_edge() {
        let square = Square {
            center: Vec2::ZERO,
            half_size: 10.0,
        };
        let ray = Ray::new(Vec2::new(10.0, 0.0), Vec2::new(1.0, 0.0));

        // Ray starting on square edge
        let hit = square.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_zero_length_ray_direction() {
        let ray = Ray::new(Vec2::ZERO, Vec2::ZERO);
        // Should handle gracefully (normalize will produce NaN or zero)
        assert!(ray.direction.length().is_nan() || ray.direction == Vec2::ZERO);
    }

    // Complex polygon tests
    #[test]
    fn test_ray_hits_concave_polygon() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(10.0, 0.0),
                Vec2::new(15.0, 5.0),
                Vec2::new(12.0, 3.0), // Creates concave shape
                Vec2::new(15.0, -5.0),
            ],
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = polygon.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_hits_pentagon() {
        let polygon = Polygon {
            vertices: vec![
                Vec2::new(10.0, 0.0),
                Vec2::new(12.0, 3.0),
                Vec2::new(11.0, 6.0),
                Vec2::new(9.0, 6.0),
                Vec2::new(8.0, 3.0),
            ],
        };
        let ray = Ray::new(Vec2::new(10.0, -5.0), Vec2::new(0.0, 1.0));

        let hit = polygon.raycast(&ray);
        assert!(hit.is_some());
    }

    // Angle tests
    #[test]
    fn test_ray_at_45_degrees() {
        let circle = Circle {
            center: Vec2::new(10.0, 10.0),
            radius: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 1.0).normalize());

        let hit = circle.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_at_shallow_angle() {
        let circle = Circle {
            center: Vec2::new(100.0, 1.0),
            radius: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.01).normalize());

        let hit = circle.raycast(&ray);
        assert!(hit.is_some());
    }

    // Negative direction tests
    #[test]
    fn test_ray_negative_direction() {
        let circle = Circle {
            center: Vec2::new(-10.0, 0.0),
            radius: 3.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(-1.0, 0.0));

        let hit = circle.raycast(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_downward() {
        let square = Square {
            center: Vec2::new(0.0, -10.0),
            half_size: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(0.0, -1.0));

        let hit = square.raycast(&ray);
        assert!(hit.is_some());

        let hit = hit.unwrap();
        assert!((hit.distance - 5.0).abs() < 0.001);
    }

    // Distance accuracy tests
    #[test]
    fn test_raycast_distance_accuracy() {
        let circle = Circle {
            center: Vec2::new(100.0, 0.0),
            radius: 10.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = circle.raycast(&ray).unwrap();
        assert!((hit.distance - 90.0).abs() < 0.001);
    }

    #[test]
    fn test_raycast_hit_point_on_surface() {
        let circle = Circle {
            center: Vec2::new(10.0, 0.0),
            radius: 5.0,
        };
        let ray = Ray::new(Vec2::ZERO, Vec2::new(1.0, 0.0));

        let hit = circle.raycast(&ray).unwrap();
        let distance_from_center = hit.point.distance(circle.center);
        assert!((distance_from_center - circle.radius).abs() < 0.001);
    }
}
