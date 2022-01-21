use vulkan_renderer::{
    device::VDevice, framebuffer::VFramebuffers, instance::VInstance,
    physical_device::VPhysicalDevice, surface::VSurface, swapchain::VSwapchain,
};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

fn main() {
    let event_loop = EventLoop::new();
    let instance = VInstance::new("Sample", 0).expect("Failed to create instance.");
    let surface =
        VSurface::new(instance.instance(), &event_loop).expect("Failed to create surface.");

    let physical_device =
        VPhysicalDevice::new(&instance, &surface).expect("Failed to create physical device.");
    let device = VDevice::new(&physical_device).expect("Failed to create device.");
    let swapchain = VSwapchain::new(&device).expect("Failed to create swapchain.");
    let framebuffers =
        VFramebuffers::new(&device, swapchain.get_image_views(), surface.dimensions())
            .expect("Failed to create framebuffers.");

    event_loop.run(|event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event:
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
