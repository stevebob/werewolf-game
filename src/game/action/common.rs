use ecs::*;
use game::*;
use game::data::*;
use spatial_hash::*;
use direction::Direction;
use coord::Coord;

pub fn walk(action: &mut EcsAction, entity: EntityRef, direction: Direction) -> Result<()> {
    let current_position = entity.position().ok_or(Error::MissingComponent)?;
    let new_position = current_position + direction.vector();
    action.insert_position(entity.id(), new_position);

    Ok(())
}

pub fn open_door(action: &mut EcsAction, door: EntityRef) -> Result<()> {
    action.remove_solid(door.id());
    action.insert_opacity(door.id(), 0.0);
    action.insert_door_state(door.id(), DoorState::Open);

    action.insert_tile(door.id(), TileType::OpenDoor);

    action.set_action_description(ActionDescription {
        message: ActionMessageType::PlayerOpenDoor,
        coord: door.position().ok_or(Error::MissingComponent)?,
    });

    Ok(())
}

pub fn close_door(action: &mut EcsAction, door: EntityRef) -> Result<()> {
    action.insert_solid(door.id());
    action.insert_opacity(door.id(), 1.0);
    action.insert_door_state(door.id(), DoorState::Closed);

    action.insert_tile(door.id(), TileType::ClosedDoor);

    action.set_action_description(ActionDescription {
        message: ActionMessageType::PlayerCloseDoor,
        coord: door.position().ok_or(Error::MissingComponent)?,
    });

    Ok(())
}

pub fn close(action: &mut EcsAction, entity_id: EntityId, direction: Direction) -> Result<()> {
    action.set_close(Close::new(entity_id, direction));
    Ok(())
}

pub fn fire_bullet(action: &mut EcsAction,
                   entity: EntityRef,
                   delta: Coord,
                   ids: &EntityIdReserver) -> Result<()> {

    const SPEED_CELLS_PER_SEC: f64 = 40.0;

    let mut velocity = RealtimeVelocity::new(delta, SPEED_CELLS_PER_SEC);
    let firer_position = entity.position().ok_or(Error::MissingComponent)?;
    let bullet_position = firer_position + velocity.step_in_place();

    prototypes::bullet(action.entity_mut(ids.new_id()), bullet_position, velocity);

    Ok(())
}

pub fn realtime_velocity_move(action: &mut EcsAction, entity: EntityRef, velocity: RealtimeVelocity) -> Result<()> {

    let current_position = entity.position().ok_or(Error::MissingComponent)?;
    let current_velocity = entity.realtime_velocity().ok_or(Error::MissingComponent)?;

    let (new_velocity, offset) = current_velocity.step();

    action.insert_realtime_velocity(entity.id(), new_velocity);
    action.insert_position(entity.id(), current_position + offset);

    action.set_action_time_ms(velocity.ms_per_cell());

    Ok(())
}

pub fn destroy(action: &mut EcsAction, entity: EntityRef) -> Result<()> {

    action.remove_entity(entity);

    Ok(())
}

pub fn move_clouds(action: &mut EcsAction, entity_id: EntityId, ecs: &EcsCtx, spatial_hash: &SpatialHashTable) -> Result<()> {

    let mut cloud_state = ecs.cloud_state_borrow_mut(entity_id).ok_or(Error::MissingComponent)?;

    cloud_state.progress(1.0);

    for (coord, cell) in izip!(spatial_hash.coord_iter(), spatial_hash.cell_iter()) {
        let moon = !cloud_state.is_cloud(coord);
        if cell.outside() && cell.moon() != moon {
            if moon {
                action.insert_moon(cell.any_outside().ok_or(Error::MissingComponent)?);
            } else {
                action.remove_moon(cell.any_outside().ok_or(Error::MissingComponent)?);
            }
        }
    }

    Ok(())
}

pub fn level_switch(action: &mut EcsAction, level_switch: LevelSwitch) -> Result<()> {

    action.set_level_switch_action(level_switch);

    Ok(())
}

pub fn projectile_collision(action: &mut EcsAction, projectile_collision: ProjectileCollision) -> Result<()> {

    action.set_projectile_collision(projectile_collision);
    action.set_no_commit();

    Ok(())
}

pub fn damage(action: &mut EcsAction, to_damage: EntityRef, amount: usize) -> Result<()> {

    let mut hit_points = to_damage.hit_points().ok_or(Error::MissingComponent)?;

    hit_points.dec(amount);

    action.insert_hit_points(to_damage.id(), hit_points);

    Ok(())
}

pub fn die(action: &mut EcsAction, entity: EntityRef) -> Result<()> {

    let ticket = entity.schedule_ticket().ok_or(Error::MissingComponent)?;
    action.set_schedule_invalidate(ticket.sequence_no);

    action.remove_entity(entity);

    Ok(())
}
