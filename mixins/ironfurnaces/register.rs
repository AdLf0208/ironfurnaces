// Declare `/ironfurnaces info <pos>`.
fn observe_register(commands: &mut Vec<NativeCommand>) {
    crate::game::ironfurnaces_state::on_register_commands(commands);
}
