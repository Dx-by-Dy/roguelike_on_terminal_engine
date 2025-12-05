use engine::addr;
use engine::component::transformation::Transformation;
use engine::ui::terminal::TerminalI;
use engine::ui::user_event_gateway::UserEventGatewayI;
use engine::{
    component::{point::Point, size::Size, timestamp::Timestamp},
    data_master::DataMasterI,
    game::Game,
    transformation_master::TransformationMasterI,
    ui::configs::{MountingPoint, MountingPointKind, SurfaceConfig, UIConfig},
};
use roguelike::player::{player, player_move};

fn start(game: &mut Game) {
    player(game);
    let pointer = game
        .set(Transformation::new(listener))
        .set_name(addr!(listener))
        .finalize();
    game.add_transformation(Timestamp::ZERO, pointer);
}

fn listener(game: &mut Game) {
    match game.get_one_event_and_drain() {
        Some(event) => match event {
            engine::component::user_event::UserEvent::KeyEvent(_) => {
                let pointer = game.get_pointer_by_name(addr!(player_move)).unwrap();
                game.set_by_pointer(pointer, event);
                game.call(pointer);
            }
            engine::component::user_event::UserEvent::Resize(_) => {}
        },
        None => {}
    }
    game.add_transformation(game.current_timestamp().add(10), game.current_pointer());
}

fn main() {
    let ui_config = UIConfig {
        terminal_size: Size::new(80, 40),
    };

    let mut game = Game::new(ui_config);

    game.add_surface(SurfaceConfig::new(
        Size::new(80, 40),
        MountingPoint::new(Point::new(0, 0), MountingPointKind::UpperLeftCorner),
    ));

    start(&mut game);
    game.run();

    // for y in 0..20 {
    //     for x in 0..20 {
    //         let unit = match (x, y) {
    //             (0, 0) => DrawUnit::new('█', UnitModificator::DFLT_UNIT_MODIFICATOR),
    //             (0, val) if val == 20 - 1 => {
    //                 DrawUnit::new('█', UnitModificator::DFLT_UNIT_MODIFICATOR)
    //             }
    //             (val, 0) if val == 20 - 1 => {
    //                 DrawUnit::new('█', UnitModificator::DFLT_UNIT_MODIFICATOR)
    //             }
    //             (valx, valy) if valx == 20 - 1 && valy == 20 - 1 => {
    //                 DrawUnit::new('█', UnitModificator::DFLT_UNIT_MODIFICATOR)
    //             }
    //             (0, _) => DrawUnit::new('│', UnitModificator::DFLT_UNIT_MODIFICATOR),
    //             (val, _) if val == 20 - 1 => {
    //                 DrawUnit::new('│', UnitModificator::DFLT_UNIT_MODIFICATOR)
    //             }
    //             (_, 0) => DrawUnit::new('—', UnitModificator::DFLT_UNIT_MODIFICATOR),
    //             (_, val) if val == 20 - 1 => {
    //                 DrawUnit::new('—', UnitModificator::DFLT_UNIT_MODIFICATOR)
    //             }
    //             _ => DrawUnit::new(' ', UnitModificator::DFLT_UNIT_MODIFICATOR),
    //         };
    //         if unit.symbol() != ' ' {
    //             game.update_surface(SurfacePosition { x, y }, 0, unit);
    //         }
    //     }
    // }

    // game.redraw();

    // sleep(std::time::Duration::from_secs(10));

    // loop {
    //     sleep(Duration::from_millis(3000));

    //     while let Some(event) = game.get_user_event() {
    //         match event {
    //             UserEvent::KeyEvent(key_event) => {
    //                 println!("{:?}", key_event);
    //                 if key_event.code == KeyCode::Esc {
    //                     return Ok(());
    //                 }
    //             }
    //             UserEvent::Resize(new_size) => println!("Resize {:?}", new_size),
    //         }
    //     }
    // }
}
