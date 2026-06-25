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
    physics: PhysicsWorldRef,
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

    fn rebuild_spatial_index(&mut self) {
        for cell in self.grid.values_mut() {
            cell.clear();
        }

        for (idx, opt_obj) in self.entities.iter().enumerate() {
            if let Some(obj) = opt_obj {
                let (min_x, min_y) = self.to_grid_coord(Vec2::new(
                    obj.last_position.x - obj.last_radius,
                    obj.last_position.y - obj.last_radius,
                ));
                let (max_x, max_y) = self.to_grid_coord(Vec2::new(
                    obj.last_position.x + obj.last_radius,
                    obj.last_position.y + obj.last_radius,
                ));

                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        self.grid.entry((x, y)).or_insert_with(Vec::new).push(idx);
                    }
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

        obj.last_position = obj.instance.position(obj.rigidbody.as_ref());
        obj.last_radius = obj.instance.radius();

        commands.append(&mut state.command_buffer);
    }

    pub fn update_and_draw_inner(&mut self, camera_min: Vec2, camera_max: Vec2) {
        let (min_cell_x, min_cell_y) = self.to_grid_coord(camera_min);
        let (max_cell_x, max_cell_y) = self.to_grid_coord(camera_max);

        let mut updated_indices = vec![false; self.entities.len()];
        let mut visible_indices = Vec::new();
        let mut global_commands = Vec::new();

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

        self.rebuild_spatial_index();
    }

    pub fn delete_entity(&mut self, idx: usize) {
        if idx < self.entities.len() {
            let Some(entity) = self.entities[idx].take() else {
                return;
            };

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

        let entity = SomeEntity2D {
            instance: entity,
            last_position: pos,
            last_radius: radius,
            rigidbody,
        };

        if let Some(vacant_idx) = self.free_list.pop() {
            self.entities[vacant_idx] = Some(entity);
        } else {
            self.entities.push(Some(entity));
        }
    }
}
