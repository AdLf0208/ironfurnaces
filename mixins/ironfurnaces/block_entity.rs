// A tier furnace block gets its block entity.
fn observe_block_entity(block: &str, entity: &mut Option<(String, horizon_world::nbt::Compound)>) {
    crate::game::ironfurnaces_state::on_block_entity(block, entity);
}
