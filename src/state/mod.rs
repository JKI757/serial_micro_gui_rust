use serialport::{available_ports};

struct SerialPorts{
    ports : serialport::Result<Vec<serialport::SerialPortInfo>>,
    current_port : serialport::SerialPortInfo,
}
