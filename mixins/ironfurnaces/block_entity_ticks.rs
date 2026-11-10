// A tier furnace block entity ticks on the server.
fn observe_block_entity_ticks(block: &str, ticks: &mut bool) {
    crate::game::ironfurnaces_state::on_block_entity_ticks(block, ticks);
}
