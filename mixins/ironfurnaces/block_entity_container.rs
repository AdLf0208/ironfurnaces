// A tier furnace block entity has input/fuel/output slots and the vanilla
// furnace menu.
fn observe_block_entity_container(
    block: &str,
    container: &mut Option<horizon_client::containers::ModContainer>,
) {
    crate::game::ironfurnaces_state::on_block_entity_container(block, container);
}
