use std::fs;
use star_over_bethlehem::{BackendType, DeviceCreateDesc, Instance, InstanceCreateDesc, ShaderKind, ShaderModuleCreateDesc, ShaderSource};

pub fn main() {
    let instance = Instance::new(&InstanceCreateDesc {
        api_validation: false,
        backend_type: BackendType::Metal,
    })
    .unwrap();

    let physical_devices = instance.get_physical_devices();
    let physical_device = &physical_devices[0];

    println!("Enumerated adapter: {}", physical_device.name());

    let device = instance
        .create_device(&DeviceCreateDesc {
            physical_device: physical_device.clone(),
        })
        .unwrap();

    let mesh_shader_source = fs::read_to_string("examples/mesh_shader.hlsl")
        .unwrap();

    let mesh_shader = device.create_shader(&ShaderModuleCreateDesc {
        name: "Mesh shader example".into(),
        source: ShaderSource::Hlsl {
            source: mesh_shader_source.into(),
            defines: vec![],
        },
        kind: ShaderKind::Mesh,
        entry_point: "ms_main".into(),
    }).unwrap();
}
