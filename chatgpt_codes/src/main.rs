use std::{
    env,
    ffi::{CStr, CString},
    io::{self, Write},
    os::unix::io::AsRawFd,
    ptr,
};

fn main() -> io::Result<()> {
    // Obtenha os argumentos da linha de comando
    let args: Vec<String> = env::args().collect();

    // Verifique se foram fornecidos dois argumentos (nome da rede e senha)
    if args.len() != 3 {
        writeln!(
            io::stderr(),
            "Uso: {} <nome da rede> <senha>",
            args[0]
        )?;
        return Ok(());
    }

    // Obtenha o nome da rede e a senha
    let ssid = &args[1];
    let password = &args[2];

    // Crie uma estrutura de configuração da rede
    let mut network_config: wifi_rs::network::NetworkConfig = Default::default();
    network_config.ssid = CString::new(ssid.as_bytes()).unwrap();
    network_config.password = CString::new(password.as_bytes()).unwrap();

    // Obtenha uma interface WiFi
    let interface = wifi_rs::interface::Interface::get_first().unwrap();

    // Obtenha o descriptor de arquivo da interface
    let fd = interface.as_raw_fd();

    // Abra a interface WiFi
    let mut iface: wifi_rs::interface::nl80211::Iface =
        unsafe { wifi_rs::interface::nl80211::Iface::open(fd).unwrap() };

    // Habilite o modo AP (ponto de acesso) na interface
    iface.ap_enable().unwrap();

    // Defina a configuração da rede
    iface.set_network_config(network_config).unwrap();

    println!("Rede WiFi criada com sucesso!");

    Ok(())
}
