use vpsadmin_client::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = std::env::args()
        .nth(1)
        .ok_or("usage: blocking_vps VPS_ID")?;
    let base = std::env::var("VPSADMIN_URL").unwrap_or_else(|_| "https://api.vpsfree.cz".into());
    let api = Client::builder(base)
        .token(std::env::var("VPSADMIN_TOKEN")?)
        .build()?;
    let vps = api.vps().show(id).send()?.output;
    println!("{}", vps.hostname.as_deref().unwrap_or("<not returned>"));
    Ok(())
}
