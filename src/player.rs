use engine::{
    addr,
    component::{
        positions::SurfacePosition,
        ref_to_surface::RefToSurface,
        transformation::Transformation,
        user_event::{UserEvent, key_event::key_code::KeyCode},
    },
    data_master::DataMasterI,
    game::Game,
    transformation_master::TransformationMasterI,
    ui::{
        draw_unit::{DrawUnit, UnitModificator},
        terminal::TerminalI,
    },
};

pub fn player(game: &mut Game) {
    game.set(SurfacePosition::new(10, 10))
        .set(DrawUnit::new('@', UnitModificator::DFLT_UNIT_MODIFICATOR))
        .set(RefToSurface::new(0))
        .set_name(addr!(player))
        .finalize();
    game.set(Transformation::new(player_move))
        .set_name(addr!(player_move))
        .finalize();
    game.set(Transformation::new(player_move_up))
        .set_name(addr!(player_move_up))
        .finalize();
    game.set(Transformation::new(player_delete))
        .set_name(addr!(player_delete))
        .finalize();
    game.set(Transformation::new(player_draw))
        .set_name(addr!(player_draw))
        .finalize();

    game.call(game.get_pointer_by_name(addr!(player_draw)).unwrap());
    game.redraw();
}

/// Ожидается, что по данному указателю будет
/// `UserEvent` в котором лежит нажатие клавиши.
pub fn player_move(game: &mut Game) {
    let cp = game.current_pointer();
    match game.get::<UserEvent>(cp).unwrap() {
        UserEvent::KeyEvent(key_event) => match key_event.code {
            KeyCode::Char(ch) => match ch {
                'w' => {
                    game.call(game.get_pointer_by_name(addr!(player_move_up)).unwrap());
                }
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
    game.call(game.get_pointer_by_name(addr!(player_delete)).unwrap());

    let pos_ref =
        game.get_mut_unchecked::<SurfacePosition>(game.get_pointer_by_name(addr!(player)).unwrap());
    if pos_ref.y > 0 {
        pos_ref.y -= 1
    }

    game.call(game.get_pointer_by_name(addr!(player_draw)).unwrap());
    game.redraw();
}
