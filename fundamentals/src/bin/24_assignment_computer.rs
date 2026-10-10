// Topic: Implementing functionality with impl
struct Cpu {
    cores: u32,
    ghz: f64,
}

impl Cpu {
    fn new(cores: u32, ghz: f64) -> Self {
        Self {
            cores: cores,
            ghz: ghz,
        }
    }

    fn print(&self) {
        println!("Cores: {} GHz: {}", self.cores, self.ghz);
    }
}
// 2. OS enum:
#[allow(dead_code)]
enum OS {
    Linux,
    Windows,
    MacOS,
}
// 3. Computer struct and impl:

struct Computer {
    cpu: Cpu,
    os: OS,
    ram_gb: u32,
}

impl Computer {
    fn new(cpu: Cpu, os: OS, ram_gb: u32) -> Self {
        Self {
            cpu: cpu,
            os: os,
            ram_gb: ram_gb,
        }
    }
    fn print(&self) {
        self.cpu.print();
        match self.os {
            OS::Linux => println!("Linux"),
            OS::Windows => println!("Windows"),
            OS::MacOS => println!("Mac OS"),
        }
        println!("Ram GB: {}", self.ram_gb);
    }
}

fn main() {
    let amd = Cpu::new(8, 2.0);
    let yoga = Computer::new(amd, OS::Linux, 32,);
    yoga.print()

}