use crate::{
    command_pool::VCommandPools,
    physical_device::VPhysicalDevice,
    queue_family::{VQueueFamilyIndices, VQueueType, VQueues},
    RendererResult,
};
use ash::{
    extensions::khr::Swapchain,
    vk::{CommandPool, DeviceCreateInfo, DeviceQueueCreateInfo, Queue},
    Device, Instance,
};
use std::collections::HashSet;

pub struct VDevice<'a> {
    device: Device,
    queues: VQueues,
    command_pools: VCommandPools,
    physical_device: &'a VPhysicalDevice<'a>,
}

impl<'a> VDevice<'a> {
    pub fn new(physical_device: &'a VPhysicalDevice) -> RendererResult<Self> {
        let queue_infos = Self::device_queue_create_infos(physical_device.queue_family_indices());
        let extensions = [Swapchain::name().as_ptr()];
        let device_create_info = Self::device_create_info(&queue_infos, &extensions);
        let device = unsafe {
            physical_device.instance().create_device(
                physical_device.physical_device(),
                &device_create_info,
                None,
            )?
        };

        let queues = VQueues::new(&device, physical_device.queue_family_indices());
        let command_pools = VCommandPools::new(&device, physical_device.queue_family_indices())?;
        Ok(Self {
            device,
            queues,
            physical_device,
            command_pools,
        })
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn get_queue(&self, queue_type: VQueueType) -> Queue {
        self.queues.get_queue(queue_type)
    }

    pub fn physical_device(&self) -> &VPhysicalDevice {
        self.physical_device
    }

    pub fn instance(&self) -> &Instance {
        self.physical_device.instance()
    }

    pub fn get_command_pool(&self, queue_type: VQueueType) -> CommandPool {
        self.command_pools.get_command_pool(queue_type)
    }

    fn device_create_info(
        queue_infos: &[DeviceQueueCreateInfo],
        extensions: &[*const i8],
    ) -> DeviceCreateInfo {
        DeviceCreateInfo {
            queue_create_info_count: queue_infos.len() as u32,
            p_queue_create_infos: queue_infos.as_ptr(),
            enabled_extension_count: extensions.len() as u32,
            pp_enabled_extension_names: extensions.as_ptr(),
            ..Default::default()
        }
    }

    // This makes no sense probably
    fn device_queue_create_infos(
        queue_family_indices: VQueueFamilyIndices,
    ) -> Vec<DeviceQueueCreateInfo> {
        let unique_indices = HashSet::<u32>::from_iter([
            queue_family_indices.compute,
            queue_family_indices.graphics,
            queue_family_indices.present,
        ]);
        unique_indices
            .iter()
            .map(|&queue_family_index| DeviceQueueCreateInfo {
                p_queue_priorities: [1.0].as_ptr(),
                queue_family_index,
                queue_count: 1,
                ..Default::default()
            })
            .collect()
    }

    #[allow(dead_code)]
    fn get_device_extensions(physical_device: &VPhysicalDevice) -> RendererResult<()> {
        let extension_props = unsafe {
            physical_device
                .instance()
                .enumerate_device_extension_properties(physical_device.physical_device())?
        };
        println!("{:#?}", extension_props);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{VDevice, VPhysicalDevice};
    use crate::{instance::VInstance, queue_family::VQueueType, surface::VSurface, RendererResult};
    use ash::vk::Handle;
    use winit::platform::windows::EventLoopExtWindows;

    #[test]
    fn creates_device() -> RendererResult<()> {
        let instance = VInstance::create("Test", 0)?;

        #[cfg(target_os = "windows")]
        {
            let surface = VSurface::create_surface(
                instance.instance(),
                &EventLoopExtWindows::new_any_thread(),
            )?;
            let physical_device = VPhysicalDevice::new(&instance, &surface)?;
            let device = VDevice::new(&physical_device)?;

            // Queues
            assert_ne!(device.queues.compute.as_raw(), 0);
            assert_ne!(device.queues.graphics.as_raw(), 0);
            assert_ne!(device.queues.present.as_raw(), 0);

            // Command Pools
            assert_ne!(
                device
                    .command_pools
                    .get_command_pool(VQueueType::Compute)
                    .as_raw(),
                0
            );
            assert_ne!(
                device
                    .command_pools
                    .get_command_pool(VQueueType::Graphics)
                    .as_raw(),
                0
            );
            assert_ne!(
                device
                    .command_pools
                    .get_command_pool(VQueueType::Present)
                    .as_raw(),
                0
            );
        }
        Ok(())
    }
}
