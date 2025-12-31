use engine::{
    addr,
    component::{
        positions::SurfacePosition,
        ref_to_surface::RefToSurface,
        user_event::{UserEvent, key_event::key_code::KeyCode},
    },
    data_master::DataMasterI,
    game::Game,
    ui::{
        draw_unit::{DrawUnit, UnitModificator},
        terminal::TerminalI,
    },
};

#[engine_macros::register_gtr(init_gtr)]
pub fn player(game: &mut Game) {
    game.set(SurfacePosition::new(10, 10))
        .set(DrawUnit::new('@', UnitModificator::DFLT_UNIT_MODIFICATOR))
        .set(RefToSurface::new(0))
        .set_name(addr!(player))
        .finalize();

    player_draw(game);
    game.redraw();
}

/// Ожидается, что по данному указателю будет
/// `UserEvent` в котором лежит нажатие клавиши.
#[engine_macros::register_gtr(set_in_datamaster)]
pub fn player_move(game: &mut Game) {
    let cp = game.get_pointer_by_name_unchecked(addr!(player_move));
    match game.get::<UserEvent>(cp).unwrap() {
        UserEvent::KeyEvent(key_event) => match key_event.code {
            KeyCode::Char(ch) => match ch {
                'w' => player_move_up(game),
                _ => {}
            },
            _ => {}
        },
        UserEvent::Resize(_) => {}
    }
}

pub fn player_delete(game: &mut Game) {
    let player = game.get_pointer_by_name(addr!(player)).unwrap();
    let pos = game.get_unchecked::<SurfacePosition>(player);
    let surface_ref = game.get_unchecked::<RefToSurface>(player);
    game.degrade_surface(*pos, *surface_ref).unwrap();
}

pub fn player_draw(game: &mut Game) {
    let player = game.get_pointer_by_name(addr!(player)).unwrap();
    let pos = game.get_unchecked::<SurfacePosition>(player);
    let surface_ref = game.get_unchecked::<RefToSurface>(player);
    let unit = game.get_unchecked::<DrawUnit>(player);
    game.update_surface(*pos, *surface_ref, *unit).unwrap();
}

pub fn player_move_up(game: &mut Game) {
    player_delete(game);

    let pos =
        game.get_mut_unchecked::<SurfacePosition>(game.get_pointer_by_name(addr!(player)).unwrap());
    if pos.y > 0 {
        pos.y -= 1
    }

    player_draw(game);
    game.redraw();
}
