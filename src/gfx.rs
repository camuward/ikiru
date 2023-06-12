//! Non-emulation graphics code.
use std::cell::Cell;
use std::ffi::CString;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use ash::{vk, Entry};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::app::emu;
use crate::emu::{EmuParams, Emulator};

/// Creates the emulation window.
pub fn spawn(params: EmuParams) -> eyre::Result<emu::Window> {
    // create a channel to send an error from the vulkan thread
    let (tx, rx) = oneshot::channel::<eyre::Result<()>>();
    let emu = Arc::new(Emulator::start(params));

    // spawn a thread to run the vulkan event loop
    let thread = thread::spawn({
        let _emu = Arc::clone(&emu);
        let tx = Cell::new(Some(tx));

        move || {
            let catch_err = || -> eyre::Result<()> {
                // create info for vulkan instance
                let app_info = vk_app_info();
                let info = vk::InstanceCreateInfo::builder().application_info(&app_info);

                // load vulkan and create instance
                let entry = unsafe { Entry::load()? };
                unsafe { entry.create_instance(&info, None)? };

                // setup event loop
                let event_loop = EventLoop::new();
                let _window = WindowBuilder::new()
                    .with_title("Vulkan")
                    .build(&event_loop)?;

                // ok, we're done
                tx.take().unwrap().send(Ok(())).unwrap();

                // hand over control flow to the event loop
                event_loop.run(move |event, _, control_flow| match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                })
            };

            // run setup, will only return if there's an error
            if let Err(e) = catch_err() {
                // send the error to the main thread
                tx.take().unwrap().send(Err(e)).unwrap();
            }

            // kill this thread
            panic!();
        }
    });

    // wait for the vulkan thread to finish initializing
    let result = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("took >5s to init vulkan");

    // if it errored, kill the vulkan thread
    match result {
        Err(e) => {
            thread.join().unwrap_err();
            Err(e)
        }
        Ok(()) => Ok(emu::Window { thread, emu }),
    }
}

fn vk_app_info() -> vk::ApplicationInfo {
    use crate::{VER_MAJOR, VER_MINOR, VER_PATCH};

    vk::ApplicationInfo::builder()
        .application_name(CString::new(env!("CARGO_PKG_NAME")).unwrap().as_c_str())
        .application_version(vk::make_api_version(0, VER_MAJOR, VER_MINOR, VER_PATCH))
        .engine_name(CString::new("No Engine").unwrap().as_c_str())
        .engine_version(vk::make_api_version(0, 1, 0, 0))
        .api_version(vk::make_api_version(0, 1, 0, 0))
        .build()
}
