use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use mdns_sd::{ResolvedService, ServiceDaemon, ServiceEvent};
use std::{collections::HashMap, net::Ipv4Addr, time::Duration};
use tokio::time::timeout;

pub async fn run_mdns_scan(timeout_secs: u64) {
    let mdns = ServiceDaemon::new().expect("Failed to start the mDNS daemon");

    // Main services announced by printers
    let service_types = [
        "_ipp._tcp.local.",            // Internet Printing Protocol (AirPrint, etc.)
        "_pdl-datastream._tcp.local.", // HP JetDirect / Raw Port 9100
        "_printer._tcp.local.",        // Line Printer Daemon (LPD)
        "_scanner._tcp.local.",        // Network Scanners
    ];

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Box<ResolvedService>>(100);

    for st in service_types {
        if let Ok(receiver) = mdns.browse(st) {
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                while let Ok(event) = receiver.recv_async().await {
                    if let ServiceEvent::ServiceResolved(info) = event {
                        let _ = tx_clone.send(info).await;
                    }
                }
            });
        }
    }

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.yellow} {msg}")
            .unwrap(),
    );
    pb.set_message("Scanning for network printers...");
    pb.enable_steady_tick(Duration::from_millis(100));

    let mut found_printers = HashMap::new();

    let _ = timeout(Duration::from_secs(timeout_secs), async {
        while let Some(info) = rx.recv().await {
            let hostname = info.host.trim_end_matches(".local.").to_string();

            let ip = info
                .addresses
                .iter()
                .find_map(|scoped_ip| scoped_ip.to_string().parse::<Ipv4Addr>().ok());

            if let Some(ipv4) = ip {
                found_printers.insert(ipv4, hostname);
            }
        }
    })
    .await;

    pb.finish_and_clear();

    if found_printers.is_empty() {
        println!("No printers found on the local network.");
    } else {
        println!("\n✅ {}\n", "Discovered printers:".green().bold());

        println!(
            "      {:<15} {} {}",
            "IP Address".bold(),
            "│".dimmed(),
            "Hostname".bold()
        );

        println!(
            "{}",
            " ─────────────────────┼──────────────────────".dimmed()
        );

        let mut sorted_printers: Vec<_> = found_printers.into_iter().collect();
        sorted_printers.sort_by_key(|(ip, _)| *ip);

        for (ip, hostname) in sorted_printers {
            println!(
                " 🖨️   {:<15} {} {}",
                ip.to_string().cyan().bold(),
                "│".dimmed(),
                hostname
            );
        }
    }
}
