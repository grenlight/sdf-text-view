
extern crate raw_window_handle;

pub struct AppView {
    pub view: winit::window::Window,
    pub scale_factor: f32,
    pub device: wgpu::Device,
    pub surface: wgpu::Surface,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
}

impl AppView {
    pub fn new(view: winit::window::Window) -> Self {
        let scale_factor = view.hidpi_factor();
        let physical = view.inner_size().to_physical(scale_factor);
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: physical.width as u32,
            height: physical.height as u32,
            present_mode: wgpu::PresentMode::Vsync,
        };
        
        let device = get_device();
        let surface = wgpu::Surface::create(&view);
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        AppView {
            view,
            scale_factor: scale_factor as f32,
            device,
            surface,
            sc_desc,
            swap_chain,
        }
    }
}

fn get_device() -> wgpu::Device {
    let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        backends: wgpu::BackendBit::PRIMARY,
    }).unwrap();
    adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    })
}

impl crate::GPUContext for AppView {
    fn update_swap_chain(&mut self) {
        let size = self.get_view_size();
        self.sc_desc.width = size.width;
        self.sc_desc.height = size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    fn get_view_size(&self) -> crate::ViewSize {
        let scale_factor = self.view.hidpi_factor();
        // let physical = size.to_physical(scale_factor);
        let physical = self.view.inner_size().to_physical(scale_factor);

        crate::ViewSize {
            width: physical.width as u32,
            height: physical.height as u32,
        }
    }
}