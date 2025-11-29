use sysinfo::{System, Networks, Disks};
use tiny_http::{Server, Response, StatusCode};
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use std::env;

struct Config {
    host: String,
    port: u16,
    path: String,
    interval: u64,
}

impl Config {
    fn from_env() -> Self {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);
        let path = env::var("PATH_STATS").unwrap_or_else(|_| "/stats".to_string());
        let interval = env::var("CACHE_INTERVAL")
            .unwrap_or_else(|_| "1".to_string())
            .parse()
            .unwrap_or(1);
        
        // 命令行参数优先级更高
        let args: Vec<String> = env::args().collect();
        let mut cfg = Config { host, port, path, interval };
        
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--host" | "-h" => {
                    if i + 1 < args.len() {
                        cfg.host = args[i + 1].clone();
                        i += 2;
                    } else { i += 1; }
                }
                "--port" | "-p" => {
                    if i + 1 < args.len() {
                        cfg.port = args[i + 1].parse().unwrap_or(cfg.port);
                        i += 2;
                    } else { i += 1; }
                }
                "--path" => {
                    if i + 1 < args.len() {
                        cfg.path = args[i + 1].clone();
                        i += 2;
                    } else { i += 1; }
                }
                "--interval" | "-i" => {
                    if i + 1 < args.len() {
                        cfg.interval = args[i + 1].parse().unwrap_or(cfg.interval);
                        i += 2;
                    } else { i += 1; }
                }
                _ => i += 1,
            }
        }
        
        cfg
    }
}

struct AppState {
    sys: System,
    networks: Networks,
    disks: Disks,
    last_update: Instant,
    cached_data: String,
    last_net_rx: u64,
    last_net_tx: u64,
    last_disk_read: u64,
    last_disk_write: u64,
    cache_interval: Duration,
}

impl AppState {
    fn new(interval: u64) -> Self {
        let mut sys = System::new_all();
        let mut networks = Networks::new_with_refreshed_list();
        let mut disks = Disks::new_with_refreshed_list();
        
        sys.refresh_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        networks.refresh(true);
        disks.refresh(true);
        
        let (net_rx, net_tx) = Self::get_net_total(&networks);
        let (disk_read, disk_write) = Self::get_disk_total(&disks);
        let cached_data = Self::format_data(&sys, 0.0, 0.0, 0.0, 0.0);
        
        AppState {
            sys,
            networks,
            disks,
            last_update: Instant::now(),
            cached_data,
            last_net_rx: net_rx,
            last_net_tx: net_tx,
            last_disk_read: disk_read,
            last_disk_write: disk_write,
            cache_interval: Duration::from_secs(interval),
        }
    }

    fn get_net_total(networks: &Networks) -> (u64, u64) {
        let mut rx = 0u64;
        let mut tx = 0u64;
        for (_name, data) in networks {
            rx += data.total_received();
            tx += data.total_transmitted();
        }
        (rx, tx)
    }

    fn get_disk_total(disks: &Disks) -> (u64, u64) {
        let mut read = 0u64;
        let mut write = 0u64;
        for disk in disks {
            read += disk.usage().total_read_bytes;
            write += disk.usage().total_written_bytes;
        }
        (read, write)
    }

    fn format_speed(bytes_per_sec: f64) -> String {
        if bytes_per_sec >= 1_000_000.0 {
            format!("{:.1}MB/s", bytes_per_sec / 1_000_000.0)
        } else {
            format!("{:.1}KB/s", bytes_per_sec / 1_000.0)
        }
    }

    fn format_data(sys: &System, io_read_rate: f64, io_write_rate: f64, 
                   net_rx_rate: f64, net_tx_rate: f64) -> String {
        let cpu = sys.global_cpu_usage();
        let mem = (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0;
        let swap = if sys.total_swap() > 0 {
            (sys.used_swap() as f64 / sys.total_swap() as f64) * 100.0
        } else {
            0.0
        };

        format!(
            "cpu=[{:.1}]\nmem=[{:.0}]\nswap=[{:.1}]\nio_read=[{}]\nio_write=[{}]\nnet_rx=[{}]\nnet_tx=[{}]",
            cpu, mem, swap,
            Self::format_speed(io_read_rate),
            Self::format_speed(io_write_rate),
            Self::format_speed(net_rx_rate),
            Self::format_speed(net_tx_rate)
        )
    }

    fn get_data(&mut self) -> String {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        
        if elapsed >= self.cache_interval.as_secs_f64() {
            self.sys.refresh_cpu_usage();
            self.sys.refresh_memory();
            self.networks.refresh(true);
            self.disks.refresh(true);
            
            let (net_rx, net_tx) = Self::get_net_total(&self.networks);
            let (disk_read, disk_write) = Self::get_disk_total(&self.disks);
            
            let net_rx_rate = (net_rx - self.last_net_rx) as f64 / elapsed;
            let net_tx_rate = (net_tx - self.last_net_tx) as f64 / elapsed;
            let io_read_rate = (disk_read - self.last_disk_read) as f64 / elapsed;
            let io_write_rate = (disk_write - self.last_disk_write) as f64 / elapsed;
            
            self.cached_data = Self::format_data(
                &self.sys, io_read_rate, io_write_rate, 
                net_rx_rate, net_tx_rate
            );
            
            self.last_net_rx = net_rx;
            self.last_net_tx = net_tx;
            self.last_disk_read = disk_read;
            self.last_disk_write = disk_write;
            self.last_update = now;
        }
        
        self.cached_data.clone()
    }
}

fn main() {
    let config = Config::from_env();
    let addr = format!("{}:{}", config.host, config.port);
    
    let state = Arc::new(Mutex::new(AppState::new(config.interval)));
    let server = Server::http(&addr).unwrap();
    
    println!("rainmeter-remote-simple-metric-server serving at: http://{}{}", addr, config.path);
    println!("Config: host={}, port={}, path={}, interval={}s", 
             config.host, config.port, config.path, config.interval);

    for request in server.incoming_requests() {
        if request.url() == config.path {
            let data = state.lock().unwrap().get_data();
            let response = Response::from_string(data)
                .with_header(tiny_http::Header::from_bytes(
                    &b"Content-Type"[..], 
                    &b"text/plain; charset=utf-8"[..]
                ).unwrap())
                .with_header(tiny_http::Header::from_bytes(
                    &b"Server"[..],
                    &b"rainmeter-remote-simple-metric-server"[..]
                ).unwrap());
            request.respond(response).ok();
        } else {
            let html = format!(
                "<!DOCTYPE HTML PUBLIC \"-//IETF//DTD HTML 2.0//EN\">\n\
                <html><head>\n\
                <title>404 Not Found</title>\n\
                </head><body>\n\
                <h1>Not Found</h1>\n\
                <p>The requested URL {} was not found on this server.</p>\n\
                <p>please visit {}  <p>
                <hr>\n\
                <address>rainmeter-remote-simple-metric-server Server at {} Port {}</address>\n\
                </body></html>",
                request.url(), config.path, config.host, config.port
            );
            let response = Response::from_string(html)
                .with_status_code(StatusCode(404))
                .with_header(tiny_http::Header::from_bytes(
                    &b"Content-Type"[..],
                    &b"text/html; charset=utf-8"[..]
                ).unwrap())
                .with_header(tiny_http::Header::from_bytes(
                    &b"Server"[..],
                    &b"rainmeter-remote-simple-metric-server"[..]
                ).unwrap());
            request.respond(response).ok();
        }
    }
}