// Right-click with an upgrade item: swap the furnace for the next tier before
// the furnace opens its menu.
fn observe_right_click_block(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    face: [i32; 3],
    canceled: &mut bool,
) {
    crate::game::ironfurnaces_state::on_right_click_block(game, pos, face, canceled);
}
