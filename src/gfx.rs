//! Non-emulation graphics code.
use std::cell::Cell;
use std::ffi::CString;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use ash::{vk, Entry};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

/// Creates the emulation window.
pub fn spawn() -> eyre::Result<JoinHandle<()>> {
    let (err_tx, err_rx) = oneshot::channel();

    // spawn a thread to run the vulkan event loop
    let thread = thread::spawn(move || {
        let err_tx = Cell::new(Some(err_tx));

        let catch_err = || -> eyre::Result<()> {
            // create info for vulkan instance
            let app_info = vk::ApplicationInfo::builder()
                .application_name(CString::new("My Vulkan App")?.as_c_str())
                .application_version(vk::make_api_version(0, 1, 0, 0))
                .engine_name(CString::new("No Engine")?.as_c_str())
                .engine_version(vk::make_api_version(0, 1, 0, 0))
                .api_version(vk::make_api_version(0, 1, 0, 0))
                .build();
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
            err_tx.take().unwrap().send(Ok(())).unwrap();

            // run event loop
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

        // send the error to the main thread
        if let Err(e) = catch_err() {
            err_tx.take().unwrap().send(Err(e)).unwrap();
        }

        // either we caught an error or event_loop.run() somehow returned
        panic!();
    });

    // wait for the vulkan thread to finish initializing
    err_rx
        .recv_timeout(Duration::from_secs(5))
        .expect("took >5s to init vulkan")
        .map(|_| thread)
}
