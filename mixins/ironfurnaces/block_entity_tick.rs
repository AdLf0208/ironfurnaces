// One server tick of a tier furnace: smelt, burn fuel, set `lit`.
fn observe_block_entity_tick(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    entity: &mut horizon_client::containers::ModBlockEntity,
) {
    crate::game::ironfurnaces_state::on_block_entity_tick(game, pos, entity);
}
