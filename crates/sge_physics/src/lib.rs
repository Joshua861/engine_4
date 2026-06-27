use std::collections::HashMap;

use player::{Player, PlayerKey};
use rapier2d::prelude::*;
use sge_api::shapes_2d::*;
use sge_color::Color;
use sge_macros::gen_ref_type;
use sge_rendering::{d2::Renderer2D, dq2d, wdq2d};
use sge_time::physics_delta_time;
use sge_vectors::Vec2;
use slotmap::{SlotMap, new_key_type};

pub mod player;

const PIXELS_PER_METRE: f32 = 100.0;

fn to_rapier(v: Vec2) -> Vector<Real> {
    vector![v.x / PIXELS_PER_METRE, -v.y / PIXELS_PER_METRE]
}

fn from_rapier(v: &Vector<Real>) -> Vec2 {
    Vec2::new(v.x * PIXELS_PER_METRE, -v.y * PIXELS_PER_METRE)
}

fn pos_to_rapier(v: Vec2) -> Isometry<Real> {
    Isometry::translation(v.x / PIXELS_PER_METRE, -v.y / PIXELS_PER_METRE)
}

fn pos_from_rapier(iso: &Isometry<Real>) -> Vec2 {
    Vec2::new(
        iso.translation.x * PIXELS_PER_METRE,
        -iso.translation.y * PIXELS_PER_METRE,
    )
}

#[derive(Clone, Debug)]
pub enum Bounds {
    Point,
    /// full size from center
    Rect(Vec2),
    Circle(f32),
    Capsule {
        half_height: f32,
        radius: f32,
    },
    CapsuleX {
        half_width: f32,
        radius: f32,
    },
    Triangle(Vec2, Vec2, Vec2),
    /// build from point cloud
    ConvexHull(Vec<Vec2>),
    Polyline(Vec<Vec2>),
    Line {
        a: Vec2,
        b: Vec2,
    },
    Compound(Vec<(Vec2, Box<Bounds>)>),
}

impl Bounds {
    pub fn rect(w: f32, h: f32) -> Self {
        Self::Rect(Vec2::new(w, h))
    }

    pub fn circle(radius: f32) -> Self {
        Self::Circle(radius)
    }

    pub fn capsule(half_height: f32, radius: f32) -> Self {
        Self::Capsule {
            half_height,
            radius,
        }
    }

    pub fn capsule_x(half_width: f32, radius: f32) -> Self {
        Self::CapsuleX { half_width, radius }
    }

    pub fn triangle(a: Vec2, b: Vec2, c: Vec2) -> Self {
        Self::Triangle(a, b, c)
    }

    pub fn convex_hull(points: impl Into<Vec<Vec2>>) -> Self {
        Self::ConvexHull(points.into())
    }

    pub fn polyline(points: impl Into<Vec<Vec2>>) -> Self {
        Self::Polyline(points.into())
    }

    pub fn line(a: Vec2, b: Vec2) -> Self {
        Self::Line { a, b }
    }

    pub fn compound(children: impl Into<Vec<(Vec2, Bounds)>>) -> Self {
        Self::Compound(
            children
                .into()
                .into_iter()
                .map(|(offset, b)| (offset, Box::new(b)))
                .collect(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct ColliderConfig {
    pub restitution: f32,
    pub friction: f32,
    pub density: f32,
    pub sensor: bool,
}

impl Default for ColliderConfig {
    fn default() -> Self {
        Self {
            restitution: 0.0,
            friction: 0.5,
            density: 1.0,
            sensor: false,
        }
    }
}

impl ColliderConfig {
    pub fn restitution(mut self, v: f32) -> Self {
        self.restitution = v;
        self
    }

    pub fn friction(mut self, v: f32) -> Self {
        self.friction = v;
        self
    }

    pub fn density(mut self, v: f32) -> Self {
        self.density = v;
        self
    }

    pub fn sensor(mut self, v: bool) -> Self {
        self.sensor = v;
        self
    }
}

fn vec2_to_point(v: Vec2) -> rapier2d::math::Point<Real> {
    rapier2d::math::Point::new(v.x / PIXELS_PER_METRE, -v.y / PIXELS_PER_METRE)
}

fn bounds_to_rapier_collider(
    bounds: &Bounds,
    cfg: &ColliderConfig,
) -> rapier2d::geometry::Collider {
    let base = bounds_to_builder(bounds);
    base.restitution(cfg.restitution)
        .friction(cfg.friction)
        .density(cfg.density)
        .sensor(cfg.sensor)
        .active_events(ActiveEvents::COLLISION_EVENTS)
        .build()
}

fn bounds_to_builder(bounds: &Bounds) -> ColliderBuilder {
    match bounds {
        Bounds::Point => ColliderBuilder::ball(0.001 / PIXELS_PER_METRE),

        Bounds::Rect(size) => ColliderBuilder::cuboid(
            size.x * 0.5 / PIXELS_PER_METRE,
            size.y * 0.5 / PIXELS_PER_METRE,
        ),

        Bounds::Circle(r) => ColliderBuilder::ball(r / PIXELS_PER_METRE),

        Bounds::Capsule {
            half_height,
            radius,
        } => ColliderBuilder::capsule_y(half_height / PIXELS_PER_METRE, radius / PIXELS_PER_METRE),

        Bounds::CapsuleX { half_width, radius } => {
            ColliderBuilder::capsule_x(half_width / PIXELS_PER_METRE, radius / PIXELS_PER_METRE)
        }

        Bounds::Triangle(a, b, c) => {
            let pa = vec2_to_point(*a);
            let pb = vec2_to_point(*b);
            let pc = vec2_to_point(*c);
            ColliderBuilder::triangle(pa, pb, pc)
        }

        Bounds::ConvexHull(pts) => {
            let rapier_pts: Vec<_> = pts.iter().map(|v| vec2_to_point(*v)).collect();
            ColliderBuilder::convex_hull(&rapier_pts).expect(
                "ConvexHull: could not compute convex hull (need at least 3 non-collinear points)",
            )
        }

        Bounds::Polyline(pts) => {
            let rapier_pts: Vec<_> = pts.iter().map(|v| vec2_to_point(*v)).collect();
            let indices: Vec<[u32; 2]> = (0..rapier_pts.len() as u32 - 1)
                .map(|i| [i, i + 1])
                .collect();
            ColliderBuilder::polyline(rapier_pts, Some(indices))
        }

        Bounds::Line { a, b } => ColliderBuilder::segment(vec2_to_point(*a), vec2_to_point(*b)),

        Bounds::Compound(children) => {
            let shapes: Vec<(Isometry<Real>, SharedShape)> = children
                .iter()
                .map(|(offset, child_bounds)| {
                    let iso = Isometry::translation(
                        offset.x / PIXELS_PER_METRE,
                        -offset.y / PIXELS_PER_METRE,
                    );
                    let shape = bounds_to_builder(child_bounds)
                        .build()
                        .shared_shape()
                        .clone();
                    (iso, shape)
                })
                .collect();
            ColliderBuilder::compound(shapes)
        }
    }
}

fn draw_bounds(pos: Vec2, bounds: &Bounds, color: Color, thickness: f32, renderer: Renderer2D) {
    match bounds {
        Bounds::Point => draw_circle_to(pos, 2.0, color, renderer),
        Bounds::Circle(r) => draw_circle_outline_to(pos, *r, color, thickness, renderer),
        Bounds::Rect(size) => {
            draw_rect_outline_to(pos - *size * 0.5, *size, thickness, color, renderer)
        }
        Bounds::Capsule {
            half_height,
            radius,
        } => {
            draw_rect_outline_to(
                pos - Vec2::new(*radius, half_height + radius),
                Vec2::new(radius * 2.0, (half_height + radius) * 2.0),
                thickness,
                color,
                renderer,
            );
            draw_circle_outline_to(
                pos + Vec2::new(0.0, *half_height),
                *radius,
                color,
                thickness,
                renderer,
            );
            draw_circle_outline_to(
                pos - Vec2::new(0.0, *half_height),
                *radius,
                color,
                thickness,
                renderer,
            );
        }
        Bounds::CapsuleX { half_width, radius } => {
            draw_rect_outline_to(
                pos - Vec2::new(half_width + radius, *radius),
                Vec2::new((half_width + radius) * 2.0, radius * 2.0),
                thickness,
                color,
                renderer,
            );
            draw_circle_outline_to(
                pos + Vec2::new(*half_width, 0.0),
                *radius,
                color,
                thickness,
                renderer,
            );
            draw_circle_outline_to(
                pos - Vec2::new(*half_width, 0.0),
                *radius,
                color,
                thickness,
                renderer,
            );
        }
        Bounds::Triangle(a, b, c) => {
            let pts = [pos + *a, pos + *b, pos + *c];
            for i in 0..3 {
                draw_line_to(pts[i], pts[(i + 1) % 3], thickness, color, renderer);
            }
        }
        Bounds::ConvexHull(pts) if pts.len() >= 2 => {
            let world_pts: Vec<_> = pts.iter().map(|p| pos + *p).collect();
            for i in 0..world_pts.len() {
                draw_line_to(
                    world_pts[i],
                    world_pts[(i + 1) % world_pts.len()],
                    thickness,
                    color,
                    renderer,
                );
            }
        }
        Bounds::Polyline(pts) if pts.len() >= 2 => {
            for i in 0..pts.len() - 1 {
                draw_line_to(pos + pts[i], pos + pts[i + 1], thickness, color, renderer);
            }
        }
        Bounds::Line { a, b } => {
            draw_line_to(pos + *a, pos + *b, thickness, color, renderer);
        }
        Bounds::Compound(children) => {
            for (offset, child) in children {
                draw_bounds(pos + *offset, child, color, thickness, renderer);
            }
        }
        _ => {}
    }
}

gen_ref_type!(PhysicsWorld, PhysicsWorldRef, worlds);

new_key_type! {
    pub struct ObjectKey;
}

pub struct ObjectHandles {
    pub rigid_body: RigidBodyHandle,
    pub collider: rapier2d::geometry::ColliderHandle,
    pub bounds: Bounds,
    pub is_dynamic: bool,
}

pub struct PhysicsWorld {
    id: usize,
    gravity: f32,

    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,

    objects: SlotMap<ObjectKey, ObjectHandles>,
    players: SlotMap<PlayerKey, Player>,
    handle_to_key: HashMap<RigidBodyHandle, ObjectKey>,

    collisions: HashMap<ObjectKey, Vec<CollisionInfo>>,
}

#[derive(Clone, Copy, Debug)]
pub struct CollisionInfo {
    pub other: PhysicsObjectRef,
    pub points: CollisionPoints,
    pub event: CollisionType,
}

#[derive(Clone, Copy, Debug)]
pub enum CollisionType {
    Started,
    Ongoing,
    Stopped,
}

impl CollisionType {
    pub fn is_colliding(&self) -> bool {
        matches!(self, Self::Started | Self::Ongoing)
    }
}

impl PhysicsWorld {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> PhysicsWorldRef {
        Self {
            id: get_worlds_state().len(),
            gravity: 980.0,

            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),

            objects: SlotMap::with_key(),
            players: SlotMap::with_key(),
            handle_to_key: HashMap::new(),
            collisions: HashMap::new(),
        }
        .create()
    }

    fn get_ref(&self) -> PhysicsWorldRef {
        PhysicsWorldRef(self.id)
    }

    pub fn update(&mut self) {
        let dt = physics_delta_time();
        let gravity = vector![0.0, -self.gravity / PIXELS_PER_METRE];

        self.update_players();

        let integration_parameters = IntegrationParameters {
            dt,
            ..Default::default()
        };

        let (collision_send, collision_recv) = std::sync::mpsc::channel();
        let (contact_force_send, _contact_force_recv) = std::sync::mpsc::channel();
        let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);

        self.physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &(),
            &event_handler,
        );

        self.rebuild_collisions(collision_recv);
    }

    fn rebuild_collisions(&mut self, collision_recv: std::sync::mpsc::Receiver<CollisionEvent>) {
        for infos in self.collisions.values_mut() {
            infos.retain(|i| !matches!(i.event, CollisionType::Stopped));
            for info in infos.iter_mut() {
                if matches!(info.event, CollisionType::Started) {
                    info.event = CollisionType::Ongoing;
                }
            }
        }
        self.collisions.retain(|_, v| !v.is_empty());

        let world_ref = PhysicsWorldRef(self.id);

        while let Ok(event) = collision_recv.try_recv() {
            match event {
                CollisionEvent::Started(ch1, ch2, _) => {
                    let rb1 = self.collider_set[ch1].parent();
                    let rb2 = self.collider_set[ch2].parent();
                    let (rb1, rb2) = match (rb1, rb2) {
                        (Some(a), Some(b)) => (a, b),
                        _ => continue,
                    };
                    let key1 = match self.handle_to_key.get(&rb1) {
                        Some(k) => *k,
                        None => continue,
                    };
                    let key2 = match self.handle_to_key.get(&rb2) {
                        Some(k) => *k,
                        None => continue,
                    };
                    let points = self.compute_collision_points(ch1, ch2, key1, key2);
                    self.collisions
                        .entry(key1)
                        .or_default()
                        .push(CollisionInfo {
                            other: PhysicsObjectRef {
                                world: world_ref,
                                key: key2,
                            },
                            points,
                            event: CollisionType::Started,
                        });
                    self.collisions
                        .entry(key2)
                        .or_default()
                        .push(CollisionInfo {
                            other: PhysicsObjectRef {
                                world: world_ref,
                                key: key1,
                            },
                            points: invert(points),
                            event: CollisionType::Started,
                        });
                }
                CollisionEvent::Stopped(ch1, ch2, _) => {
                    let rb1 = self.collider_set[ch1].parent();
                    let rb2 = self.collider_set[ch2].parent();
                    let (rb1, rb2) = match (rb1, rb2) {
                        (Some(a), Some(b)) => (a, b),
                        _ => continue,
                    };
                    let key1 = match self.handle_to_key.get(&rb1) {
                        Some(k) => *k,
                        None => continue,
                    };
                    let key2 = match self.handle_to_key.get(&rb2) {
                        Some(k) => *k,
                        None => continue,
                    };
                    if let Some(infos) = self.collisions.get_mut(&key1)
                        && let Some(info) = infos.iter_mut().find(|i| i.other.key == key2)
                    {
                        info.event = CollisionType::Stopped;
                    }
                    if let Some(infos) = self.collisions.get_mut(&key2)
                        && let Some(info) = infos.iter_mut().find(|i| i.other.key == key1)
                    {
                        info.event = CollisionType::Stopped;
                    }
                }
            }
        }
    }

    fn compute_collision_points(
        &self,
        ch1: rapier2d::geometry::ColliderHandle,
        ch2: rapier2d::geometry::ColliderHandle,
        key1: ObjectKey,
        key2: ObjectKey,
    ) -> CollisionPoints {
        if let Some(pair) = self.narrow_phase.contact_pair(ch1, ch2) {
            for manifold in &pair.manifolds {
                let normal_rapier = manifold.data.normal;
                let normal = Vec2::new(normal_rapier.x, -normal_rapier.y);
                let depth = manifold
                    .points
                    .iter()
                    .filter(|p| p.dist < 0.0)
                    .map(|p| -p.dist * PIXELS_PER_METRE)
                    .fold(0.0_f32, f32::max);
                if depth > 0.0 {
                    return CollisionPoints {
                        normal,
                        depth,
                        collision: true,
                    };
                }
            }
        }

        let pos1 = self.get_position(key1);
        let pos2 = self.get_position(key2);
        let delta = pos1 - pos2;
        let dist = delta.length();
        let normal = if dist > 0.0 { delta / dist } else { Vec2::X };
        CollisionPoints {
            normal,
            depth: 0.0,
            collision: true,
        }
    }

    fn insert_object(
        &mut self,
        bounds: Bounds,
        rigid_body: RigidBody,
        cfg: ColliderConfig,
    ) -> ObjectKey {
        let is_dynamic = rigid_body.is_dynamic();
        let rb_handle = self.rigid_body_set.insert(rigid_body);

        let rapier_collider = bounds_to_rapier_collider(&bounds, &cfg);
        let col_handle = self.collider_set.insert_with_parent(
            rapier_collider,
            rb_handle,
            &mut self.rigid_body_set,
        );

        let key = self.objects.insert(ObjectHandles {
            rigid_body: rb_handle,
            collider: col_handle,
            bounds,
            is_dynamic,
        });
        self.handle_to_key.insert(rb_handle, key);
        key
    }

    pub fn create_dynamic(&mut self, bounds: Bounds) -> PhysicsObjectRef {
        self.create_dynamic_with(bounds, ColliderConfig::default())
    }

    pub fn create_fixed(&mut self, bounds: Bounds) -> PhysicsObjectRef {
        self.create_fixed_with(bounds, ColliderConfig::default())
    }

    pub fn create_kinematic(&mut self, bounds: Bounds) -> PhysicsObjectRef {
        self.create_kinematic_with(bounds, ColliderConfig::default())
    }

    pub fn create_dynamic_with(&mut self, bounds: Bounds, cfg: ColliderConfig) -> PhysicsObjectRef {
        let rb = RigidBodyBuilder::dynamic().build();
        let key = self.insert_object(bounds, rb, cfg);
        PhysicsObjectRef {
            world: PhysicsWorldRef(self.id),
            key,
        }
    }

    pub fn create_fixed_with(&mut self, bounds: Bounds, cfg: ColliderConfig) -> PhysicsObjectRef {
        let rb = RigidBodyBuilder::fixed().build();
        let key = self.insert_object(bounds, rb, cfg);
        PhysicsObjectRef {
            world: PhysicsWorldRef(self.id),
            key,
        }
    }

    pub fn create_kinematic_with(
        &mut self,
        bounds: Bounds,
        cfg: ColliderConfig,
    ) -> PhysicsObjectRef {
        let rb = RigidBodyBuilder::kinematic_position_based().build();
        let key = self.insert_object(bounds, rb, cfg);
        PhysicsObjectRef {
            world: PhysicsWorldRef(self.id),
            key,
        }
    }

    pub fn remove(&mut self, key: ObjectKey) {
        if let Some(handles) = self.objects.remove(key) {
            self.handle_to_key.remove(&handles.rigid_body);
            self.collider_set.remove(
                handles.collider,
                &mut self.island_manager,
                &mut self.rigid_body_set,
                true,
            );
            self.rigid_body_set.remove(
                handles.rigid_body,
                &mut self.island_manager,
                &mut self.collider_set,
                &mut self.impulse_joint_set,
                &mut self.multibody_joint_set,
                true,
            );
        }
        self.collisions.remove(&key);
    }

    pub fn set_gravity(&mut self, gravity: f32) {
        self.gravity = gravity;
    }
    pub fn get_gravity(&self) -> f32 {
        self.gravity
    }
    pub fn get_gravity_mut(&mut self) -> &mut f32 {
        &mut self.gravity
    }

    fn get_position(&self, key: ObjectKey) -> Vec2 {
        let rb_handle = self.objects[key].rigid_body;
        pos_from_rapier(self.rigid_body_set[rb_handle].position())
    }

    fn get_rotation(&self, key: ObjectKey) -> f32 {
        let rb_handle = self.objects[key].rigid_body;
        -self.rigid_body_set[rb_handle].rotation().angle()
    }

    fn set_rotation(&mut self, key: ObjectKey, angle: f32) {
        let rb_handle = self.objects[key].rigid_body;
        let pos = self.rigid_body_set[rb_handle].position().translation.vector;
        let iso = Isometry::new(pos, -angle);
        self.rigid_body_set[rb_handle].set_position(iso, true);
    }

    fn set_position(&mut self, key: ObjectKey, pos: Vec2) {
        let rb_handle = self.objects[key].rigid_body;
        self.rigid_body_set[rb_handle].set_position(pos_to_rapier(pos), true);
    }

    fn get_velocity(&self, key: ObjectKey) -> Vec2 {
        let rb_handle = self.objects[key].rigid_body;
        from_rapier(self.rigid_body_set[rb_handle].linvel())
    }

    fn set_velocity(&mut self, key: ObjectKey, vel: Vec2) {
        let rb_handle = self.objects[key].rigid_body;
        self.rigid_body_set[rb_handle].set_linvel(to_rapier(vel), true);
    }

    fn add_velocity(&mut self, key: ObjectKey, vel: Vec2) {
        let current = self.get_velocity(key);
        self.set_velocity(key, current + vel);
    }

    fn get_angvel(&self, key: ObjectKey) -> f32 {
        let rb_handle = self.objects[key].rigid_body;
        -self.rigid_body_set[rb_handle].angvel()
    }

    fn set_angvel(&mut self, key: ObjectKey, angvel: f32) {
        let rb_handle = self.objects[key].rigid_body;
        self.rigid_body_set[rb_handle].set_angvel(-angvel, true);
    }

    fn add_angvel(&mut self, key: ObjectKey, angvel: f32) {
        let current = self.get_angvel(key);
        self.set_angvel(key, current + angvel);
    }

    fn add_force(&mut self, key: ObjectKey, force: Vec2) {
        let rb_handle = self.objects[key].rigid_body;
        self.rigid_body_set[rb_handle].add_force(to_rapier(force), true);
    }

    fn get_mass(&self, key: ObjectKey) -> f32 {
        let rb_handle = self.objects[key].rigid_body;
        self.rigid_body_set[rb_handle].mass()
    }

    fn move_by(&mut self, key: ObjectKey, offset: Vec2) {
        let current = self.get_position(key);
        self.set_position(key, current + offset);
    }

    fn get_bounds(&self, key: ObjectKey) -> &Bounds {
        &self.objects[key].bounds
    }

    fn is_dynamic(&self, key: ObjectKey) -> bool {
        self.objects[key].is_dynamic
    }

    pub fn draw_colliders(&self) {
        self.draw_colliders_to(dq2d());
    }

    pub fn draw_colliders_world(&self) {
        self.draw_colliders_to(wdq2d());
    }

    fn draw_colliders_to(&self, renderer: Renderer2D) {
        for handles in self.objects.values() {
            let pos = pos_from_rapier(self.rigid_body_set[handles.rigid_body].position());
            draw_bounds(pos, &handles.bounds, Color::RED_500, 1.5, renderer);
        }

        for (a, infos) in &self.collisions {
            let a = &self.objects[*a];
            let pos = pos_from_rapier(self.rigid_body_set[a.rigid_body].position());

            for info in infos {
                let display_length = if info.points.depth > 0.0 {
                    info.points.depth
                } else {
                    60.0
                };
                draw_arrow_to(
                    pos,
                    pos + info.points.normal * display_length,
                    2.0,
                    Color::NEUTRAL_500.with_alpha(0.3),
                    renderer,
                );
            }

            if !infos.is_empty() {
                draw_bounds(pos, &a.bounds, Color::YELLOW_500, 3.0, renderer);
            }
        }
    }
}

fn invert(mut c: CollisionPoints) -> CollisionPoints {
    c.normal = -c.normal;
    c
}

#[derive(Clone, Copy, Debug)]
pub struct CollisionPoints {
    pub normal: Vec2,
    pub depth: f32,
    pub collision: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PhysicsObjectRef {
    pub world: PhysicsWorldRef,
    pub key: ObjectKey,
}

impl PhysicsObjectRef {
    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.set_position(pos);
        self
    }

    pub fn with_velocity(mut self, vel: Vec2) -> Self {
        self.set_velocity(vel);
        self
    }

    pub fn with_angvel(mut self, angvel: f32) -> Self {
        self.set_angvel(angvel);
        self
    }

    pub fn get_position(&self) -> Vec2 {
        self.world.get_position(self.key)
    }
    pub fn set_position(&mut self, pos: Vec2) {
        self.world.set_position(self.key, pos);
    }
    pub fn get_velocity(&self) -> Vec2 {
        self.world.get_velocity(self.key)
    }
    pub fn set_velocity(&mut self, vel: Vec2) {
        self.world.set_velocity(self.key, vel);
    }
    pub fn add_velocity(&mut self, vel: Vec2) {
        self.world.add_velocity(self.key, vel);
    }
    pub fn add_force(&mut self, force: Vec2) {
        self.world.add_force(self.key, force);
    }

    pub fn get_mass(&self) -> f32 {
        self.world.get_mass(self.key)
    }
    pub fn move_by(&mut self, offset: Vec2) {
        self.world.move_by(self.key, offset);
    }
    pub fn get_bounds(&self) -> &Bounds {
        self.world.get_bounds(self.key)
    }
    pub fn is_dynamic(&self) -> bool {
        self.world.is_dynamic(self.key)
    }
    pub fn get_angvel(&self) -> f32 {
        self.world.get_angvel(self.key)
    }
    pub fn set_angvel(&mut self, angvel: f32) {
        self.world.set_angvel(self.key, angvel);
    }
    pub fn add_angvel(&mut self, angvel: f32) {
        self.world.add_angvel(self.key, angvel);
    }
    pub fn get_rotation(&self) -> f32 {
        self.world.get_rotation(self.key)
    }
    pub fn set_rotation(&mut self, angle: f32) {
        self.world.set_rotation(self.key, angle);
    }
    pub fn with_rotation(mut self, angle: f32) -> Self {
        self.set_rotation(angle);
        self
    }

    pub fn collisions(&self) -> &[CollisionInfo] {
        self.world
            .collisions
            .get(&self.key)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn is_colliding(&self) -> bool {
        !self.collisions().is_empty()
    }

    pub fn is_colliding_with(&self, other: PhysicsObjectRef) -> bool {
        self.collisions().iter().any(|i| i.other == other)
    }

    pub fn check_collision_with(&self, other: PhysicsObjectRef) -> Option<CollisionPoints> {
        self.collisions()
            .iter()
            .find(|i| i.other == other)
            .map(|i| i.points)
    }

    pub fn remove(mut self) {
        self.world.remove(self.key);
    }
    pub fn key(&self) -> ObjectKey {
        self.key
    }

    pub fn rigidbody(&mut self) -> &mut RigidBody {
        let rb_handle = self.world.objects[self.key].rigid_body;
        &mut self.world.rigid_body_set[rb_handle]
    }

    pub fn with_ccd(mut self) -> Self {
        let rb_handle = self.world.objects[self.key].rigid_body;
        self.world.rigid_body_set[rb_handle].enable_ccd(true);
        self
    }
}

pub fn init() {
    init_worlds_storage();
}
