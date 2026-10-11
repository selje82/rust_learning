#[derive(Debug)]
#[allow(dead_code)]
enum Status {
    Active,
    Maintance,
    Offline,
}

#[derive(Debug)]
struct Server {
    hostname: String,
    ram_gb: i32,
    status: Status
}

impl Server {
    fn is_ready(&self) -> bool {
        if self.ram_gb >= 16 {
            match self.status {
                Status::Active => true,
                _ => false,
            }
        } else {
            false
        }
    }
}
fn main() {
    let my_servers = vec![
        Server {
            hostname: String::from("riga"),
            ram_gb: 8,
            status: Status::Active,
        },
        Server {
            hostname: String::from("dover"),
            ram_gb: 32,
            status: Status::Active,
        },
        Server {
            hostname: String::from("bangkok"),
            ram_gb: 32,
            status:Status::Maintance,
        }
    ];

    for server in &my_servers {
        if server.is_ready() {
            println!("Ready for use: {}", &server.hostname)
        }
    }
    println!("{:#?}", &my_servers);
}