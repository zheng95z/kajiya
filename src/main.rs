mod device;
mod instance;
mod logging;
mod physical_device;
mod surface;
mod swapchain;

use ash::vk;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::sync::Arc;
use swapchain::SwapchainDesc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct WindowConfig {
    width: u32,
    height: u32,
}

fn main() -> anyhow::Result<()> {
    logging::set_up_logging()?;

    let event_loop = EventLoop::new();

    let window_cfg = WindowConfig {
        width: 1280,
        height: 720,
    };

    let window = Arc::new(
        WindowBuilder::new()
            .with_title("vicki")
            .with_inner_size(winit::dpi::LogicalSize::new(
                window_cfg.width as f64,
                window_cfg.height as f64,
            ))
            .build(&event_loop)
            .expect("window"),
    );

    let instance = instance::Instance::builder()
        .required_extensions(ash_window::enumerate_required_extensions(&*window).unwrap())
        .build()?;
    let surface = surface::Surface::new(&instance, &*window)?;

    use physical_device::*;
    let physical_devices =
        enumerate_physical_devices(&instance)?.with_presentation_support(&surface);

    info!("Available physical devices: {:#?}", physical_devices);

    let physical_device = Arc::new(
        physical_devices
            .into_iter()
            .next()
            .expect("valid physical device"),
    );

    let device = device::Device::new(&physical_device)?;
    let swapchain = device.create_swapchain(
        surface,
        SwapchainDesc {
            surface_format: vk::SurfaceFormatKHR {
                format: vk::Format::B8G8R8_UNORM,
                color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
            },
            surface_resolution: vk::Extent2D {
                width: window_cfg.width,
                height: window_cfg.height,
            },
            vsync: true,
        },
    );

    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                // Application update code.
            }
            _ => (),
        }
    })
}
