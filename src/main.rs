use steamworks::{Client, PublishedFileId};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <app_id>", args[0]);
        std::process::exit(1);
    }

    let app_id: u32 = args[1].parse().expect("Invalid AppID");

    let client = Client::init_app(app_id).expect("Steam is not running or the AppID is invalid");
    let ugc = Client::ugc(&client.0);

    let req = reqwest::blocking::Client::new();

    ugc.subscribed_items().iter().for_each(|item| {
        print!("Checking item {}... ", item.0);

        let url = format!(
            "https://steamcommunity.com/sharedfiles/filedetails/?id={}",
            item.0
        );

        let res = req.get(url).send().expect("Failed to send request");

        if res.text().unwrap().contains("<div class=\"error_ctn\">") {
            println!("Unavailable, unsubscribing...");
            ugc.unsubscribe_item(PublishedFileId(item.0), |result| match result {
                Ok(_) => println!("Unsubscribed from item"),
                Err(e) => println!("Failed to unsubscribe from item: {}", e),
            });
        } else {
            println!("OK");
        }
    });
}
