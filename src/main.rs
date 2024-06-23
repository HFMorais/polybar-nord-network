use std::process::Command;

fn main() {
    let mut output = String::new();

    if let Some(network_interface) = check_connection() {
        output = format!("{}{}", output, network_interface.replace("\n", ""));
    }

    if let Some(nordvpn_status) = check_nordvpn() {
        output = format!("{} {}{}", output, "%{T2}", nordvpn_status);
    }
 
    print!("{}", output);       
}

fn check_connection() -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("nmcli -t -f TYPE,STATE dev | grep -E 'ethernet:connected|wifi:connected'")
        .output()
        .expect("Failed connection check");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("ethernet:connected") {
            let output = Command::new("sh")
                .arg("-c")
                .arg("ip route show default | head -n 1 | awk '/default/ {print $5}'")
                .output()
                .expect("Failed ethernet check");

            if output.status.success() {
                return Some(String::from_utf8_lossy(&output.stdout).to_string());
            } else {
                return Some("ethernet".to_string());
            }
        } else if stdout.contains("wifi:connected") {
            let output = Command::new("sh")
                .arg("-c")
                .arg("iw dev | grep ssid") // Using iw to get the SSID
                .output()
                .expect("Failed to fetch SSID");

            if output.status.success() {
                let ssid_output = String::from_utf8_lossy(&output.stdout);
                let ssid_columns: Vec<&str> = ssid_output.split_whitespace().collect();
                if ssid_columns.len() > 1 {
                    let ssid_value = ssid_columns[1..].join(" ");
                    return Some(ssid_value);
                }

                return Some(String::from_utf8_lossy(&output.stdout).to_string());
            }
            
            return Some("wlan".to_string());
        }
        
        return Some("offline".to_string());
    }

    None
}

fn check_nordvpn() -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("command -v nordvpn")
        .output()
        .expect("Failed to check if Nordvpn is installed");

    // Nordvpn is installed
    if output.status.success() {
        let output = Command::new("sh")
            .arg("-c")
            .arg("nordvpn status | grep 'Status: Connected'")
            .output()
            .expect("Failed nordvpn check");

        if output.status.success() {
            return Some("".to_string());
        }
    }

    None
}
