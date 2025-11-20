use engine::ui::terminal::TerminalI;
use engine::{
    component::{point::Point, size::Size, timestamp::Timestamp, transformation::Transformation},
    data_master::DataMasterI,
    game::Game,
    transformation_master::TransformationMasterI,
    ui::configs::{MountingPoint, MountingPointKind, SurfaceConfig, UIConfig},
};
use std::process::exit;

fn start(game: &mut Game) {
    let timestamp = game.current_timestamp().next();
    println!("{:?}", timestamp);
    game.add_transformation(Timestamp::new(timestamp.value + 99), game.current_pointer());
}

fn end(_game: &mut Game) {
    exit(0)
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

    let pointer = game.set(Transformation::new(start)).finalize();
    game.add_transformation(Timestamp { value: 0 }, pointer);
    let pointer = game.set(Transformation::new(end)).finalize();
    game.add_transformation(Timestamp { value: 1000 }, pointer);
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
