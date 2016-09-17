use game::{
    EntityId,
    LevelId,
    Component,
    ComponentType,
    EntityTable,
    UpdateSummary,
    AddedComponents,
    EntityWrapper,
    IterEntityRef,
    IdEntityRef,
};

use table::{
    ToType,
    TableTable,
};
use vision::Opacity;
use grid::Grid;
use geometry::Vector2;

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct SpatialHashCell {
    pub entities: HashSet<EntityId>,
    pub components: HashMap<ComponentType, usize>,
    pub opacity: f64,
    pub last_updated: u64,
}

impl Opacity for SpatialHashCell {
    fn opacity(&self) -> f64 { self.opacity }
}

impl Default for SpatialHashCell {
    fn default() -> Self {
        SpatialHashCell {
            entities: HashSet::new(),
            components: HashMap::new(),
            opacity: 0.0,
            last_updated: 0,
        }
    }
}

impl SpatialHashCell {
    pub fn has(&self, component_type: ComponentType) -> bool {
        if let Some(count) = self.components.get(&component_type) {
            *count != 0
        } else {
            false
        }
    }

    fn get_count(&self, component_type: ComponentType) -> usize {
        if let Some(count) = self.components.get(&component_type) {
            *count
        } else {
            0
        }
    }

    fn set_count(&mut self, component_type: ComponentType, new_count: usize) {
        if self.components.contains_key(&component_type) {
            *self.components.get_mut(&component_type).unwrap() = new_count;
        } else {
            self.components.insert(component_type, new_count);
        }
    }

    fn increment_count(&mut self, component_type: ComponentType) {
        let count = self.get_count(component_type);
        self.set_count(component_type, count + 1);
    }

    fn decrement_count(&mut self, component_type: ComponentType) {
        let count = self.get_count(component_type);
        self.set_count(component_type, count - 1);
    }

    fn add_entity<'a, E: IterEntityRef<'a>>(&mut self, id: EntityId, entity: E) {
        if self.entities.insert(id) {
            for component in entity.entries() {
                self.add_component(component);
            }
        }
    }

    fn remove_entity<'a, E: IdEntityRef<'a>>(&mut self, entity: E) {
        if self.entities.remove(&entity.id()) {
            for component in entity.entries() {
                self.remove_component(component);
            }
        }
    }

    fn change_component(&mut self, old: &Component, new: &Component) {
        if let &Component::Opacity(old_opacity) = old {
            if let &Component::Opacity(new_opacity) = new {
                self.opacity += new_opacity - old_opacity;
            }
        }
    }

    fn add_component(&mut self, component: &Component) {
        self.increment_count(component.to_type());

        if let &Component::Opacity(opacity) = component {
            self.opacity += opacity;
        }
    }

    fn remove_component(&mut self , component: &Component) {
        self.decrement_count(component.to_type());

        if let &Component::Opacity(opacity) = component {
            self.opacity -= opacity;
        }
    }
}


#[derive(Debug, Clone)]
pub struct SpatialHashMap<G: Grid<Item=SpatialHashCell>> {
    pub id: Option<LevelId>,
    pub grid: G,
}

impl<G: Grid<Item=SpatialHashCell>> SpatialHashMap<G> {
    pub fn new(grid: G) -> Self {
        SpatialHashMap {
            id: None,
            grid: grid,
        }
    }

    pub fn set_id(&mut self, id: LevelId) {
        self.id = Some(id);
    }

    pub fn get_unsafe(&self, coord: (isize, isize)) -> &SpatialHashCell {
        self.grid.get_unsafe(Vector2::from_tuple(coord))
    }

    pub fn get(&self, coord: (isize, isize)) -> Option<&SpatialHashCell> {
        self.grid.get(Vector2::from_tuple(coord))
    }

    fn get_mut(&mut self, coord: (isize, isize)) -> Option<&mut SpatialHashCell> {
        self.grid.get_mut(Vector2::from_tuple(coord))
    }

    fn get_mut_unsafe(&mut self, coord: (isize, isize)) -> &mut SpatialHashCell {
        self.grid.get_mut_unsafe(Vector2::from_tuple(coord))
    }

    pub fn add_entity<'a, E: IterEntityRef<'a>>(
        &mut self, id: EntityId, entity: E, turn_count: u64)
    {
        if let Some(vec) = entity.position() {
            let cell = self.get_mut_unsafe(vec.to_tuple());
            cell.add_entity(id, entity);
            cell.last_updated = turn_count;
        }
    }

    pub fn remove_entity<'a, E: IdEntityRef<'a>>(&mut self, entity: E, turn_count: u64) {
        if let Some(vec) = entity.position() {
            let cell = self.get_mut_unsafe(vec.to_tuple());
            cell.remove_entity(entity);
            cell.last_updated = turn_count;
        }
    }

    pub fn add_components<'a, 'b, E: IdEntityRef<'a>>(
        &mut self,
        entity: E,
        changes: &AddedComponents,
        turn_count: u64)
    {
        let id = entity.id();

        // position will be set to the position of entity after the change
        let position = if let Some(new_position) = changes.position() {

            // position is special as it indicates which cell to update
            if let Some(old_position) = entity.position() {
                // entity is moving from old_position to new_position
                let mut cell = self.get_mut_unsafe(old_position.to_tuple());
                cell.remove_entity(entity);
                cell.last_updated = turn_count;
            }

            // the entity's position is changing or the entity is gaining a position
            // in either case, add the entity to the position's cell
            self.get_mut_unsafe(new_position.to_tuple()).add_entity(id, entity);

            // entity will eventually end up here
            Some(new_position)
        } else if let Some(current_position) = entity.position() {
            // entity isn't moving, so use its current position
            Some(current_position)
        } else {
            // entity has no position, so the spatial hash won't be updated
            None
        };

        if let Some(position) = position {
            let mut cell = self.get_mut_unsafe(position.to_tuple());
            for (component_type, new_component) in changes.iter() {
                if *component_type == ComponentType::Position {
                    // this has already been handled
                    continue;
                }

                if let Some(ref old_component) = entity.get(*component_type) {
                    cell.change_component(old_component, new_component);
                } else {
                    // only update the component count if the component is being added
                    cell.add_component(new_component);
                }

            }
            cell.last_updated = turn_count;
        }
    }

    pub fn remove_components<'a, E: IdEntityRef<'a>>(
        &mut self,
        entity: E,
        component_types: &HashSet<ComponentType>,
        turn_count: u64)
    {
        if let Some(position) = entity.position() {
            if component_types.contains(&ComponentType::Position) {
                // removing position - remove the entity
                self.remove_entity(entity, turn_count);
            } else {
                let mut cell = self.get_mut_unsafe(position.to_tuple());
                for component_type in component_types {
                    if let Some(ref component) = entity.get(*component_type) {
                        cell.remove_component(component);
                    }
                }
                cell.last_updated = turn_count;
            }
        }
    }

    /// Update the spatial hash's metadata. This should be called before the update is applied.
    pub fn update<'a, T>(&mut self, update: &UpdateSummary, entities: &'a T, turn_count: u64)
    where T: EntityTable<'a>,
          <T as TableTable<'a, ComponentType, Component>>::Ref: IdEntityRef<'a>,
    {
        for (id, entity) in update.added_entities.iter() {
            self.add_entity(*id, entity, turn_count);
        }

        for entity_id in &update.removed_entities {
            let entity = entities.get(*entity_id).unwrap();
            self.remove_entity(entity, turn_count);
        }

        for (entity_id, changes) in &update.added_components {
            let entity = entities.get(*entity_id).unwrap();
            self.add_components(entity, changes, turn_count);
        }

        for (entity_id, component_types) in &update.removed_components {
            let entity = entities.get(*entity_id).unwrap();
            self.remove_components(entity, component_types, turn_count);
        }
    }
}