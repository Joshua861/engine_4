use rustc_hash::FxHashMap;
use sge_camera::get_camera_2d_mut;
use sge_physics::{self as phys, PhysicsObjectRef, PhysicsWorld, PhysicsWorldRef};
use sge_vectors::Vec2;

pub struct WorldState2D {
    current_entity_id: usize,
    command_buffer: Vec<Command>,
    physics: PhysicsWorldRef,
    current_object: Option<phys::PhysicsObjectRef>,
}

enum Command {
    Delete(usize),
    Spawn(Box<dyn Entity2D>, Option<phys::PhysicsObjectRef>),
}

impl WorldState2D {
    fn new(
        current_entity_id: usize,
        physics: PhysicsWorldRef,
        current_object: Option<phys::PhysicsObjectRef>,
    ) -> Self {
        Self {
            current_entity_id,
            command_buffer: Vec::new(),
            physics,
            current_object,
        }
    }

    pub fn delete_this_entity(&mut self) {
        self.command_buffer
            .push(Command::Delete(self.current_entity_id));
    }

    pub fn spawn<T: Entity2D + 'static>(&mut self, entity: T) {
        self.command_buffer
            .push(Command::Spawn(Box::new(entity), None));
    }

    pub fn rigidbody(&self) -> Option<phys::PhysicsObjectRef> {
        self.current_object
    }

    pub fn spawn_dynamic<T: Entity2D + 'static>(
        &mut self,
        entity: T,
        bounds: phys::Bounds,
    ) -> phys::PhysicsObjectRef {
        let rigidbody = self.physics.create_dynamic(bounds);
        self.command_buffer
            .push(Command::Spawn(Box::new(entity), Some(rigidbody)));
        rigidbody
    }

    pub fn spawn_fixed<T: Entity2D + 'static>(
        &mut self,
        entity: T,
        bounds: phys::Bounds,
    ) -> phys::PhysicsObjectRef {
        let rigidbody = self.physics.create_fixed(bounds);
        self.command_buffer
            .push(Command::Spawn(Box::new(entity), Some(rigidbody)));
        rigidbody
    }

    pub fn spawn_kinematic<T: Entity2D + 'static>(
        &mut self,
        entity: T,
        bounds: phys::Bounds,
    ) -> phys::PhysicsObjectRef {
        let rigidbody = self.physics.create_kinematic(bounds);
        self.command_buffer
            .push(Command::Spawn(Box::new(entity), Some(rigidbody)));
        rigidbody
    }

    pub fn spawn_dynamic_with_config<T: Entity2D + 'static>(
        &mut self,
        entity: T,
        bounds: phys::Bounds,
        config: phys::ColliderConfig,
    ) -> phys::PhysicsObjectRef {
        let rigidbody = self.physics.create_dynamic_with(bounds, config);
        self.command_buffer
            .push(Command::Spawn(Box::new(entity), Some(rigidbody)));
        rigidbody
    }

    pub fn spawn_fixed_with_config<T: Entity2D + 'static>(
        &mut self,
        entity: T,
        bounds: phys::Bounds,
        config: phys::ColliderConfig,
    ) -> phys::PhysicsObjectRef {
        let rigidbody = self.physics.create_fixed_with(bounds, config);
        self.command_buffer
            .push(Command::Spawn(Box::new(entity), Some(rigidbody)));
        rigidbody
    }

    pub fn spawn_kinematic_with_config<T: Entity2D + 'static>(
        &mut self,
        entity: T,
        bounds: phys::Bounds,
        config: phys::ColliderConfig,
    ) -> phys::PhysicsObjectRef {
        let rigidbody = self.physics.create_kinematic_with(bounds, config);
        self.command_buffer
            .push(Command::Spawn(Box::new(entity), Some(rigidbody)));
        rigidbody
    }
}

pub struct World2D {
    entities: Vec<Option<SomeEntity2D>>,
    grid: FxHashMap<(i32, i32), Vec<usize>>,
    cell_size: f32,
    free_list: Vec<usize>,
    pub physics: PhysicsWorldRef,
}

pub trait Entity2D {
    fn update_when_offscreen(&self) -> bool {
        true
    }
    fn z_index(&self) -> i32 {
        0
    }
    fn update(&mut self, state: &mut WorldState2D);
    fn draw(&self);
    fn position(&self, rigidbody: Option<&PhysicsObjectRef>) -> Vec2;
    fn radius(&self) -> f32;
}

struct SomeEntity2D {
    instance: Box<dyn Entity2D>,
    id: usize,
    last_position: Vec2,
    last_radius: f32,
    pub rigidbody: Option<phys::PhysicsObjectRef>,
}

impl World2D {
    pub fn new(cell_size: f32) -> Self {
        Self {
            entities: Vec::new(),
            grid: FxHashMap::default(),
            free_list: Vec::new(),
            cell_size,
            physics: PhysicsWorld::new(),
        }
    }

    fn entity_grid_bounds(&self, pos: Vec2, radius: f32) -> (i32, i32, i32, i32) {
        let min_x = ((pos.x - radius) / self.cell_size).floor() as i32;
        let min_y = ((pos.y - radius) / self.cell_size).floor() as i32;
        let max_x = ((pos.x + radius) / self.cell_size).floor() as i32;
        let max_y = ((pos.y + radius) / self.cell_size).floor() as i32;

        (min_x, min_y, max_x, max_y)
    }

    fn add_entity_to_grid(&mut self, id: usize) {
        let entity = self.entities[id].as_ref().unwrap();
        let (min_x, min_y, max_x, max_y) =
            self.entity_grid_bounds(entity.last_position, entity.last_radius);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                self.grid.entry((x, y)).or_insert_with(Vec::new).push(id);
            }
        }
    }

    fn remove_entity_from_grid(&mut self, id: usize, pos: Vec2, radius: f32) {
        let (min_x, min_y, max_x, max_y) = self.entity_grid_bounds(pos, radius);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if let Some(cell) = self.grid.get_mut(&(x, y)) {
                    cell.retain(|&i| i != id);
                }
            }
        }
    }

    fn to_grid_coord(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
        )
    }

    pub fn update(&mut self) {
        self.physics.update();
        let camera = get_camera_2d_mut();
        let (min, max) = camera.visible_bounds();

        self.update_and_draw_inner(min, max);
    }

    fn update_entity(&mut self, id: usize, commands: &mut Vec<Command>) {
        let obj = self.entities[id].as_mut().unwrap();
        let mut state = WorldState2D::new(id, self.physics, obj.rigidbody);
        obj.instance.update(&mut state);

        let old_pos = obj.last_position;
        let old_radius = obj.last_radius;

        obj.last_position = obj.instance.position(obj.rigidbody.as_ref());
        obj.last_radius = obj.instance.radius();

        let has_changed = old_pos != obj.last_position || old_radius != obj.last_radius;
        if has_changed {
            let id = obj.id;
            self.remove_entity_from_grid(id, old_pos, old_radius);
            self.add_entity_to_grid(id);
        }

        commands.append(&mut state.command_buffer);
    }

    pub fn update_and_draw_inner(&mut self, camera_min: Vec2, camera_max: Vec2) {
        let (min_cell_x, min_cell_y) = self.to_grid_coord(camera_min);
        let (max_cell_x, max_cell_y) = self.to_grid_coord(camera_max);

        let mut updated_indices = vec![false; self.entities.len()];
        let mut visible_indices = Vec::new();
        let mut global_commands = Vec::new();

        let width = max_cell_x - min_cell_x;
        let height = max_cell_y - min_cell_y;

        if width * height < 100_000 {
            for x in min_cell_x..=max_cell_x {
                for y in min_cell_y..=max_cell_y {
                    if let Some(entity_indices) = self.grid.get(&(x, y)) {
                        let entity_indices = entity_indices.clone();
                        for idx in entity_indices {
                            if !updated_indices[idx] && self.entities[idx].is_some() {
                                visible_indices.push(idx);

                                self.update_entity(idx, &mut global_commands);
                                updated_indices[idx] = true;
                            }
                        }
                    }
                }
            }
        } else {
            // too many tiles onscreen to loop through all of them each frame

            let entities = unsafe { &*(&self.entities as *const Vec<Option<SomeEntity2D>>) };
            for entity in entities.iter().flatten() {
                visible_indices.push(entity.id);
                self.update_entity(entity.id, &mut global_commands);
                updated_indices[entity.id] = true;
            }
        }

        for (idx, updated) in updated_indices.iter().enumerate() {
            if !*updated {
                if let Some(obj) = &self.entities[idx] {
                    if obj.instance.update_when_offscreen() {
                        self.update_entity(idx, &mut global_commands);
                    }
                }
            }
        }

        visible_indices
            .sort_unstable_by_key(|&idx| self.entities[idx].as_ref().unwrap().instance.z_index());

        for idx in visible_indices {
            if let Some(obj) = &self.entities[idx] {
                obj.instance.draw();
            }
        }

        for cmd in global_commands {
            match cmd {
                Command::Delete(id) => {
                    self.delete_entity(id);
                }
                Command::Spawn(boxed_entity, rigidbody) => {
                    self.spawn_boxed(boxed_entity, rigidbody);
                }
            }
        }
    }

    pub fn delete_entity(&mut self, idx: usize) {
        if idx < self.entities.len() {
            let Some(entity) = self.entities[idx].take() else {
                return;
            };

            self.remove_entity_from_grid(entity.id, entity.last_position, entity.last_radius);

            if let Some(rigidbody) = entity.rigidbody {
                self.physics.remove(rigidbody.key);
            }
            self.entities[idx] = None;
            self.free_list.push(idx);
        }
    }

    pub fn spawn<T: Entity2D + 'static>(&mut self, entity: T) {
        let box_entity = Box::new(entity);
        self.spawn_boxed(box_entity, None);
    }

    pub fn spawn_dynamic<T: Entity2D + 'static>(
        &mut self,
        entity: T,
        bounds: phys::Bounds,
    ) -> phys::PhysicsObjectRef {
        let box_entity = Box::new(entity);
        let rigidbody = self.physics.create_dynamic(bounds);
        self.spawn_boxed(box_entity, Some(rigidbody));
        rigidbody
    }

    pub fn spawn_fixed<T: Entity2D + 'static>(
        &mut self,
        entity: T,
        bounds: phys::Bounds,
    ) -> phys::PhysicsObjectRef {
        let box_entity = Box::new(entity);
        let rigidbody = self.physics.create_fixed(bounds);
        self.spawn_boxed(box_entity, Some(rigidbody));
        rigidbody
    }

    pub fn spawn_kinematic<T: Entity2D + 'static>(
        &mut self,
        entity: T,
        bounds: phys::Bounds,
    ) -> phys::PhysicsObjectRef {
        let box_entity = Box::new(entity);
        let rigidbody = self.physics.create_kinematic(bounds);
        self.spawn_boxed(box_entity, Some(rigidbody));
        rigidbody
    }

    pub fn spawn_dynamic_with_config<T: Entity2D + 'static>(
        &mut self,
        entity: T,
        bounds: phys::Bounds,
        config: phys::ColliderConfig,
    ) -> phys::PhysicsObjectRef {
        let box_entity = Box::new(entity);
        let rigidbody = self.physics.create_dynamic_with(bounds, config);
        self.spawn_boxed(box_entity, Some(rigidbody));
        rigidbody
    }

    pub fn spawn_fixed_with_config<T: Entity2D + 'static>(
        &mut self,
        entity: T,
        bounds: phys::Bounds,
        config: phys::ColliderConfig,
    ) -> phys::PhysicsObjectRef {
        let box_entity = Box::new(entity);
        let rigidbody = self.physics.create_fixed_with(bounds, config);
        self.spawn_boxed(box_entity, Some(rigidbody));
        rigidbody
    }

    pub fn spawn_kinematic_with_config<T: Entity2D + 'static>(
        &mut self,
        entity: T,
        bounds: phys::Bounds,
        config: phys::ColliderConfig,
    ) -> phys::PhysicsObjectRef {
        let box_entity = Box::new(entity);
        let rigidbody = self.physics.create_kinematic_with(bounds, config);
        self.spawn_boxed(box_entity, Some(rigidbody));
        rigidbody
    }

    pub fn spawn_boxed(
        &mut self,
        entity: Box<dyn Entity2D>,
        rigidbody: Option<phys::PhysicsObjectRef>,
    ) {
        let pos = entity.position(rigidbody.as_ref());
        let radius = entity.radius();

        let mut entity = SomeEntity2D {
            instance: entity,
            last_position: pos,
            last_radius: radius,
            rigidbody,
            id: 0,
        };

        let id = if let Some(vacant_idx) = self.free_list.pop() {
            entity.id = vacant_idx;
            self.entities[vacant_idx] = Some(entity);
            vacant_idx
        } else {
            entity.id = self.entities.len();
            self.entities.push(Some(entity));
            self.entities.len() - 1
        };

        self.add_entity_to_grid(id);
    }

    pub fn debug_entities(&self) {
        use sge_api::shapes_2d::*;
        use sge_color::Color;
        use sge_vectors::vec2;

        // brighten_screen(-1.0);

        let (camera_min, camera_max) = get_camera_2d_mut().visible_bounds();
        let (min_cell_x, min_cell_y) = self.to_grid_coord(camera_min);
        let (max_cell_x, max_cell_y) = self.to_grid_coord(camera_max);

        for x in min_cell_x..=max_cell_x {
            for y in min_cell_y..=max_cell_y {
                if let Some(entity_indices) = self.grid.get(&(x, y)) {
                    if !entity_indices.is_empty() {
                        draw_square_with_outline_world(
                            vec2(x as f32 * self.cell_size, y as f32 * self.cell_size),
                            self.cell_size,
                            Color::GREEN_500.with_alpha(0.05),
                            (self.cell_size / 20.0).max(2.0),
                            Color::GREEN_500.with_alpha(0.1),
                        );
                    }
                }
            }
        }

        for entity in self.entities.iter().flatten() {
            draw_circle_world(
                entity.last_position,
                entity.last_radius,
                Color::BLUE_800.with_alpha(0.5),
            );
        }

        for entity in self.entities.iter().flatten() {
            draw_circle_outline_world(
                entity.last_position,
                entity.last_radius,
                Color::BLUE_700,
                (entity.last_radius / 50.0).max(2.0),
            );
        }
    }
}
