use serde::Deserialize;
use warp::Filter;

#[derive(Deserialize)]
struct Prefix {
    ip_prefix: String,
    #[serde(rename = "region")]
    region: String,
    #[serde(rename = "service")]
    #[allow(dead_code)] // Suppress warning if unused
    service: String,
    #[serde(rename = "network_border_group")]
    #[allow(dead_code)] // Suppress warning if unused
    network_border_group: String,
}

#[derive(Deserialize)]
struct IpRanges {
    #[serde(rename = "syncToken")]
    #[allow(dead_code)] // Suppress warning if unused
    sync_token: String,
    #[serde(rename = "createDate")]
    #[allow(dead_code)] // Suppress warning if unused
    create_date: String,
    prefixes: Vec<Prefix>,
}

#[tokio::main]
async fn main() {
    let url = "https://ip-ranges.amazonaws.com/ip-ranges.json";

    println!("Starting server...");
    println!("Fetching IP ranges from: {}", url);
    println!("Listening on all interfaces at http://0.0.0.0:3030/ip-prefixes");

    // Define the Warp route with an optional query parameter for filtering
    let route = warp::path!("ip-prefixes")
        .and(warp::query::<RegionFilter>())
        .and_then(move |filter: RegionFilter| {
            let url = url.to_string();
            async move {
                match fetch_and_filter_prefixes(&url, &filter).await {
                    Ok(prefixes) => {
                        println!("Successfully fetched and filtered IP ranges.");
                        Ok::<_, warp::Rejection>(
                            warp::reply::with_header(
                                prefixes.join("\n"),
                                "Content-Type",
                                "text/plain",
                            )
                        )
                    }
                    Err(err) => {
                        eprintln!("Error fetching or processing IP ranges: {:?}", err);
                        Err(warp::reject::custom(err))
                    }
                }
            }
        });

    warp::serve(route).run(([0, 0, 0, 0], 3030)).await;
}

// Region filter structure for query parameters
#[derive(Deserialize)]
struct RegionFilter {
    region: Option<String>,
}

// Fetch and filter IP prefixes based on the region filter
async fn fetch_and_filter_prefixes(url: &str, filter: &RegionFilter) -> Result<Vec<String>, FetchError> {
    let response = reqwest::get(url).await.map_err(|err| {
        eprintln!("Failed to fetch JSON file: {}", err);
        FetchError
    })?;
    
    println!("Response status: {}", response.status());

    let data: IpRanges = response.json().await.map_err(|err| {
        eprintln!("Failed to parse JSON: {}", err);
        FetchError
    })?;

    // Filter the prefixes based on the region filter
    let prefixes = data
        .prefixes
        .into_iter()
        .filter(|p| match &filter.region {
            Some(filter_region) => matches_region(&p.region, filter_region),
            None => true, // No filter applied
        })
        .map(|p| p.ip_prefix)
        .collect();

    Ok(prefixes)
}

// Helper function to match region with filter
fn matches_region(region: &str, filter: &str) -> bool {
    if filter.starts_with('!') {
        // Inverted match: Exclude regions that match the pattern
        let pattern = &filter[1..];
        !wildcard_match(region, pattern)
    } else {
        // Standard match: Include regions that match the pattern
        wildcard_match(region, filter)
    }
}

// Simple wildcard matching function
fn wildcard_match(value: &str, pattern: &str) -> bool {
    let pattern = regex::escape(pattern).replace(r"\*", ".*");
    let re = regex::Regex::new(&format!("^{}$", pattern)).unwrap();
    re.is_match(value)
}

#[derive(Debug)]
struct FetchError;

impl warp::reject::Reject for FetchError {}

