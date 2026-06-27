use std::{
    f32::consts::FRAC_PI_4,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

use nalgebra::{Vector2, vector};
use rapier2d::{control::KinematicCharacterController, math::Isometry, prelude::QueryFilter};
use sge_input::{Button, KeyCode, button_held, button_pressed};
use sge_time::{physics_delta_time, time};
use sge_utils::RotatingArray;
use sge_vectors::Vec2;
use slotmap::new_key_type;

new_key_type! {
    pub struct PlayerKey;
}

use crate::*;

pub struct Player {
    key: PlayerKey,
    object: ObjectKey,
    kcc: KinematicCharacterController,
    floor_max_angle: f32,
    binds: PlayerBinds,
    coyote_time: f32,
    time_last_on_ground: f32,
    velocity: Vec2,
    is_sliding_down_slope: bool,

    on_ground: RotatingArray<bool, 10>,
    position: RotatingArray<Vec2, 10>,

    num_double_jumps: usize,
    double_jumps_left: usize,

    move_speed: f32,
    jump_velocity: f32,
    double_jump_velocity: Option<f32>,

    just_jumped: bool,
    just_double_jumped: bool,

    enable_input: bool,
}

impl Player {
    fn double_jump_velocity(&self) -> f32 {
        self.double_jump_velocity.unwrap_or(self.jump_velocity)
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn set_double_jumps(&mut self, num: usize) {
        self.num_double_jumps = num;
    }

    pub fn get_double_jumps(&self) -> usize {
        self.num_double_jumps
    }

    pub fn is_in_coyote(&self) -> bool {
        let now = time();
        (now - self.time_last_on_ground) <= self.coyote_time
    }

    pub fn is_on_ground(&self) -> bool {
        self.is_really_on_ground() || self.is_in_coyote()
    }

    pub fn is_really_on_ground(&self) -> bool {
        *self.on_ground.current_value()
    }

    pub fn set_coyote_time(&mut self, ms: f32) {
        self.coyote_time = ms / 1000.0;
    }

    pub fn get_coyote_time(&self) -> f32 {
        self.coyote_time * 1000.0
    }

    pub fn can_jump(&self) -> bool {
        self.is_on_ground() || self.double_jumps_left > 0
    }

    pub fn set_max_floor_angle(&mut self, angle: f32) {
        self.floor_max_angle = angle;
    }

    pub fn time_last_on_ground(&self) -> f32 {
        self.time_last_on_ground
    }

    pub fn set_velocity(&mut self, vel: Vec2) {
        self.velocity = vel;
    }

    pub fn add_impulse(&mut self, impulse: Vec2) {
        self.velocity += impulse;
    }

    pub fn delta_translation(&self) -> Vec2 {
        self.position.current_value() - self.position.previous_value()
    }

    pub fn delta_translation_last_frame(&self) -> Vec2 {
        let a = self.position.previous_value();
        let b = self.position.past_value(2);
        a - b
    }

    pub fn is_sliding_down_slope(&self) -> bool {
        self.is_sliding_down_slope
    }

    pub fn was_on_ground_last_frame(&self) -> bool {
        *self.on_ground.previous_value()
    }

    pub fn was_on_ground(&self) -> bool {
        self.on_ground.iter().any(|&b| b)
    }

    pub fn was_in_air_last_frame(&self) -> bool {
        !self.was_on_ground_last_frame()
    }

    pub fn was_in_air(&self) -> bool {
        for i in 0..self.on_ground.len() - 1 {
            if *self.on_ground.get(i) {
                return false;
            }
        }

        true
    }

    pub fn just_landed(&self) -> bool {
        self.is_on_ground() && self.was_in_air()
    }

    pub fn double_jumps_left(&self) -> usize {
        self.double_jumps_left
    }

    pub fn move_speed(&self) -> f32 {
        self.move_speed
    }

    pub fn set_move_speed(&mut self, speed: f32) {
        self.move_speed = speed;
    }

    pub fn jump_velocity(&self) -> f32 {
        self.jump_velocity
    }

    pub fn set_jump_velocity(&mut self, velocity: f32) {
        self.jump_velocity = velocity;
    }

    pub fn set_double_jump_velocity(&mut self, velocity: f32) {
        self.double_jump_velocity = Some(velocity);
    }

    pub fn set_double_jump_velocity_none(&mut self) {
        self.double_jump_velocity = None;
    }

    pub fn just_jumped(&self) -> bool {
        self.just_jumped
    }

    pub fn just_double_jumped(&self) -> bool {
        self.just_double_jumped
    }

    pub fn is_moving(&self) -> bool {
        self.velocity != Vec2::ZERO
    }
}

#[derive(Clone, Copy)]
struct PlayerRef {
    key: PlayerKey,
    world: PhysicsWorldRef,
}

impl PlayerRef {
    pub fn object(&self) -> &ObjectHandles {
        &self.world.objects[self.object]
    }

    pub fn object_mut(&mut self) -> &mut ObjectHandles {
        let key = self.object;
        &mut self.world.objects[key]
    }
}

impl Deref for PlayerRef {
    type Target = Player;

    fn deref(&self) -> &Self::Target {
        &self.world.players[self.key]
    }
}

impl DerefMut for PlayerRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.world.players[self.key]
    }
}

pub struct PlayerController {
    value: PlayerRef,
}

impl PhysicsWorld {
    pub fn create_player_controller(&mut self, bounds: Bounds) -> PlayerController {
        let object = self.create_kinematic(bounds).key;

        let player = Player {
            #[allow(clippy::uninit_assumed_init)]
            #[allow(invalid_value)]
            key: unsafe { MaybeUninit::uninit().assume_init() },
            kcc: KinematicCharacterController::default(),
            floor_max_angle: FRAC_PI_4,
            binds: PlayerBinds::default(),
            num_double_jumps: 1,
            double_jumps_left: 1,
            coyote_time: 0.1,
            time_last_on_ground: f32::NEG_INFINITY,
            on_ground: RotatingArray::new([false; _]),
            position: RotatingArray::new([Vec2::ZERO; _]),
            velocity: Vec2::ZERO,
            object,
            double_jump_velocity: None,
            jump_velocity: 500.0,
            move_speed: 200.0,
            is_sliding_down_slope: false,
            just_jumped: false,
            just_double_jumped: false,
            enable_input: true,
        };

        let key = self.players.insert(player);
        self.players[key].key = key;

        PlayerController {
            value: PlayerRef {
                key,
                world: self.get_ref(),
            },
        }
    }

    pub(crate) fn update_players(&mut self) {
        let dt = physics_delta_time();
        let now = time();

        let keys: Vec<PlayerKey> = self.players.keys().collect();

        for pk in keys {
            self.step_player(pk, dt, now);
            self.populate_player_collisions(pk);
        }
    }

    fn populate_player_collisions(&mut self, pk: PlayerKey) {
        let p = &self.players[pk];
        let player_key = p.object;
        let collider_handle = self.objects[player_key].collider;
        let world_ref = PhysicsWorldRef(self.id);

        let contacts: Vec<(ObjectKey, CollisionPoints)> = self
            .narrow_phase
            .contact_pairs_with(collider_handle)
            .filter_map(|contact_pair| {
                let other_collider = if contact_pair.collider1 == collider_handle {
                    contact_pair.collider2
                } else {
                    contact_pair.collider1
                };
                let rb = self.collider_set[other_collider].parent()?;
                let other_key = *self.handle_to_key.get(&rb)?;
                let points = self.compute_collision_points(
                    collider_handle,
                    other_collider,
                    player_key,
                    other_key,
                );
                Some((other_key, points))
            })
            .collect();

        if let Some(old) = self.collisions.get(&player_key) {
            let old_keys: Vec<ObjectKey> = old.iter().map(|i| i.other.key).collect();
            for old_key in old_keys {
                if let Some(infos) = self.collisions.get_mut(&old_key) {
                    infos.retain(|i| i.other.key != player_key);
                }
            }
        }

        self.collisions.remove(&player_key);

        for (other_key, points) in contacts {
            self.collisions
                .entry(player_key)
                .or_default()
                .push(CollisionInfo {
                    other: PhysicsObjectRef {
                        world: world_ref,
                        key: other_key,
                    },
                    points,
                    event: CollisionType::Ongoing,
                });
            self.collisions
                .entry(other_key)
                .or_default()
                .push(CollisionInfo {
                    other: PhysicsObjectRef {
                        world: world_ref,
                        key: player_key,
                    },
                    points: invert(points),
                    event: CollisionType::Ongoing,
                });
        }
    }

    fn step_player(&mut self, pk: PlayerKey, dt: f32, now: f32) {
        let p = &mut self.players[pk];
        let binds = p.binds;

        if p.enable_input {
            let jump = button_pressed(binds.jump);
            let left = button_held(binds.left);
            let right = button_held(binds.right);

            p.velocity.y -= self.gravity * dt;
            p.velocity.x = (right as i32 - left as i32) as f32 * p.move_speed;

            p.just_double_jumped = false;
            p.just_jumped = false;

            let in_coyote = (now - p.time_last_on_ground) <= p.coyote_time;
            let can_jump = p.is_really_on_ground() || in_coyote;

            if jump {
                if can_jump {
                    p.velocity.y = p.jump_velocity;
                    p.just_jumped = true;
                    p.time_last_on_ground = f32::NEG_INFINITY;
                    p.double_jumps_left = p.num_double_jumps;
                } else if p.double_jumps_left > 0 {
                    p.velocity.y = p.double_jump_velocity();
                    p.double_jumps_left -= 1;
                    p.just_double_jumped = true;
                }
            }
        }

        let desired_translation = p.velocity * dt;
        let desired_translation: Vector2<f32> =
            vector![desired_translation.x, desired_translation.y];

        let rb_handle = self.objects[p.object].rigid_body;
        let current_pos = *self.rigid_body_set[rb_handle].position();
        let collider_handle = self.objects[p.object].collider;
        let collider_shape = self.collider_set[collider_handle].shape();
        let query_pipeline = &self.broad_phase.as_query_pipeline(
            self.narrow_phase.query_dispatcher(),
            &self.rigid_body_set,
            &self.collider_set,
            QueryFilter::default().exclude_rigid_body(rb_handle),
        );

        let movement = p.kcc.move_shape(
            dt,
            query_pipeline,
            collider_shape,
            &current_pos,
            desired_translation,
            |_| {},
        );

        p.is_sliding_down_slope = movement.is_sliding_down_slope;

        let new_pos = Isometry::translation(
            current_pos.translation.x + movement.translation.x,
            current_pos.translation.y + movement.translation.y,
        );
        self.rigid_body_set[rb_handle].set_next_kinematic_position(new_pos);

        let new_pos = crate::from_rapier(&vector![new_pos.translation.x, new_pos.translation.y]);
        p.position.push(new_pos);
        p.on_ground.push(movement.grounded);

        let just_landed = movement.grounded && p.was_in_air();

        if just_landed {
            p.double_jumps_left = p.num_double_jumps;
        }

        if movement.grounded {
            p.time_last_on_ground = now;
            p.velocity.y = 0.0;
        }
    }
}

impl PlayerController {
    pub fn set_binds(&mut self) -> PlayerBindBuilder {
        PlayerBindBuilder { value: self.value }
    }

    pub fn object(&self) -> &ObjectHandles {
        self.value.object()
    }

    pub fn object_mut(&mut self) -> &mut ObjectHandles {
        self.value.object_mut()
    }

    pub fn set_position(&mut self, pos: Vec2) {
        let rb_handle = self.object().rigid_body;
        let iso = Isometry::translation(pos.x, pos.y);
        self.value.world.rigid_body_set[rb_handle].set_next_kinematic_position(iso);
    }

    pub fn position(&self) -> Vec2 {
        let rb_handle = self.object().rigid_body;
        let iso = self.value.world.rigid_body_set[rb_handle].position();
        crate::pos_from_rapier(iso)
    }

    pub fn position_last_frame(&self) -> Vec2 {
        self.position() - self.delta_translation()
    }

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.set_position(pos);
        self
    }

    pub fn enable_input(&mut self, enable: bool) {
        self.value.enable_input = enable;
    }
}

impl Deref for PlayerController {
    type Target = Player;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for PlayerController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[derive(Clone, Copy)]
struct PlayerBinds {
    left: Button,
    right: Button,
    jump: Button,
}

impl Default for PlayerBinds {
    fn default() -> Self {
        PlayerBinds {
            left: Button::Keyboard(KeyCode::KeyA),
            right: Button::Keyboard(KeyCode::KeyD),
            jump: Button::Keyboard(KeyCode::Space),
        }
    }
}

pub struct PlayerBindBuilder {
    value: PlayerRef,
}

impl PlayerBindBuilder {
    pub fn left(mut self, button: impl Into<Button>) -> Self {
        self.value.binds.left = button.into();
        self
    }

    pub fn right(mut self, button: impl Into<Button>) -> Self {
        self.value.binds.right = button.into();
        self
    }

    pub fn jump(mut self, button: impl Into<Button>) -> Self {
        self.value.binds.jump = button.into();
        self
    }
}
