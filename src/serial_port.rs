pub fn get_list() {
        let ports = serialport::available_ports().expect("Ошибка чтения портов");
        for port in ports {
        println!("Найден порт: {:?} - {:?}", port.port_name, port.port_type);

        /*  Вывело:
        Найден порт: "COM5" - Unknown
        Найден порт: "COM1" - Unknown
        Найден порт: "COM2" - Unknown
        Найден порт: "COM3" - UsbPort(UsbPortInfo { vid: 6790, pid: 29987, serial_number: Some("5"), manufacturer: Some("wch.cn"), product: Some("USB-SERIAL CH340 (COM3)") })

        pub enum SerialPortType {
            UsbPort(UsbPortInfo),
            PciPort,
            BluetoothPort,
            Unknown,
        }

        pub struct UsbPortInfo {
            pub vid: u16,
            pub pid: u16,
            pub serial_number: Option<String>,
            pub manufacturer: Option<String>,
            pub product: Option<String>,
        }
*/
        }
}