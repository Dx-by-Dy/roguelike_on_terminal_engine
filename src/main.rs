use engine::{
    common::{point::Point, positions::SurfacePosition, size::Size},
    game::Game,
    ui::{
        configs::{MountingPoint, MountingPointKind, SurfaceConfig, TerminalConfig, UIConfig},
        draw_unit::{DrawUnit, UnitModificator},
    },
};

use std::{
    io::{self},
    thread::sleep,
};

fn main() -> io::Result<()> {
    let ui_config = UIConfig {
        terminal_config: TerminalConfig {
            terminal_size: Size::new(80, 40),
            surface_confs: vec![SurfaceConfig::new(
                Size::new(20, 20),
                MountingPoint::new(Point::new(70, 10), MountingPointKind::DownLeftCorner),
            )],
        },
    };

    let mut game = Game::new(ui_config);

    for y in 0..20 {
        for x in 0..20 {
            let unit = match (x, y) {
                (0, 0) => DrawUnit::new('█', UnitModificator::DFLT_UNIT_MODIFICATOR),
                (0, val) if val == 20 - 1 => {
                    DrawUnit::new('█', UnitModificator::DFLT_UNIT_MODIFICATOR)
                }
                (val, 0) if val == 20 - 1 => {
                    DrawUnit::new('█', UnitModificator::DFLT_UNIT_MODIFICATOR)
                }
                (valx, valy) if valx == 20 - 1 && valy == 20 - 1 => {
                    DrawUnit::new('█', UnitModificator::DFLT_UNIT_MODIFICATOR)
                }
                (0, _) => DrawUnit::new('│', UnitModificator::DFLT_UNIT_MODIFICATOR),
                (val, _) if val == 20 - 1 => {
                    DrawUnit::new('│', UnitModificator::DFLT_UNIT_MODIFICATOR)
                }
                (_, 0) => DrawUnit::new('—', UnitModificator::DFLT_UNIT_MODIFICATOR),
                (_, val) if val == 20 - 1 => {
                    DrawUnit::new('—', UnitModificator::DFLT_UNIT_MODIFICATOR)
                }
                _ => DrawUnit::new(' ', UnitModificator::DFLT_UNIT_MODIFICATOR),
            };
            if unit.symbol() != ' ' {
                game.update_surface(SurfacePosition { x, y }, 0, unit);
            }
        }
    }

    game.redraw();

    sleep(std::time::Duration::from_secs(10));

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

    Ok(())
}
