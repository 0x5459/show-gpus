use rust_gpu_tools::Device;

fn main() {
    for (i, dev) in Device::all().iter().enumerate() {
        let framework = match dev.framework() {
            #[cfg(feature = "cuda")]
            rust_gpu_tools::Framework::Cuda => "cuda",
            #[cfg(feature = "opencl")]
            rust_gpu_tools::Framework::Opencl => "opencl",
        };
        println!(
            "#{}, uuid: {}, name: {}, framework: {}, memory: {}, compute_units: {}, vendor: {}, compute_capability: {:?}",
            i,
            dev.unique_id(),
            dev.name(),
            framework,
            dev.memory(),
            dev.compute_units(),
            dev.vendor(),
            dev.compute_capability(),
        );
    }
}
