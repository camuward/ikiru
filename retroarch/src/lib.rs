use libretro_backend::{Core, CoreInfo, GameData, LoadGameResult, RuntimeHandle};

#[derive(Default)]
struct Emulator {}

impl Core for Emulator {
    fn info() -> CoreInfo {
        CoreInfo::new("ikiru", ikiru::VERSION)
    }

    fn on_load_game(&mut self, game: GameData) -> LoadGameResult {
        todo!()
    }

    fn on_unload_game(&mut self) -> GameData {
        todo!()
    }

    fn on_run(&mut self, handle: &mut RuntimeHandle) {
        todo!()
    }

    fn on_reset(&mut self) {
        todo!()
    }
}
