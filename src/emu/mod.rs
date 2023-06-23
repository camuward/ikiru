use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use derive_builder::Builder;

use crate::game::TitleId;

pub mod cafe;
pub mod cpu;
pub mod gpu;
pub mod instance;

/// A running emulator instance.
pub struct Emulator {
    title: TitleId,
    state: Mutex<EmuState>,
    instance: instance::Instance,
}

unsafe impl Send for Emulator {}
unsafe impl Sync for Emulator {}

#[derive(Debug, Clone, Builder)]
pub struct EmuParams {
    /// The folders/archives to search for game files.
    pub paths: Vec<PathBuf>,
    /// The title ID of the game to load.
    pub dlc: Vec<TitleId>,
    pub update: Option<TitleId>,
    pub title: TitleId,
}

impl Emulator {
    /// Begin emulating the provided title ID. Extracts the configuration from
    /// the provided application context.
    pub fn start(params: EmuParams) -> Self {
        Self {
            title: params.title,
            state: Mutex::new(EmuState::Paused {
                total: Duration::ZERO,
            }),
            instance: instance::Instance::new(),
        }
    }

    pub fn pause(&self) {
        let now = Instant::now();
        self.state.lock().unwrap().pause(now);
    }

    pub fn unpause(&self) {
        let now = Instant::now();
        self.state.lock().unwrap().unpause(now);
    }

    pub fn uptime(&self) -> Duration {
        self.state.lock().unwrap().uptime()
    }
}

#[derive(Debug)]
enum EmuState {
    Running {
        /// The last time the emulator was unpaused.
        unpause: Instant,
        /// The total amount of time the emulator has been unpaused, minus the
        /// time since the last unpause.
        total: Duration,
    },
    Paused {
        /// The total amount of time the emulator has been unpaused.
        total: Duration,
    },
}

impl EmuState {
    fn pause(&mut self, paused_at: Instant) {
        if let Self::Running { unpause, total } = *self {
            let time_since_unpause = paused_at.saturating_duration_since(unpause);
            let total = total + time_since_unpause;

            *self = Self::Paused { total };
        }
    }

    fn unpause(&mut self, unpause: Instant) {
        if let &mut Self::Paused { total } = self {
            *self = Self::Running { unpause, total }
        }
    }

    fn uptime(&self) -> Duration {
        match self {
            Self::Running { unpause, total } => {
                let time_since_unpause = unpause.elapsed();
                *total + time_since_unpause
            }
            Self::Paused { total } => *total,
        }
    }
}
