// Copyright (C) 2024 Bellande Robotics Sensors Research Innovation Center, Ronaldson Bellande

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use reqwest;
use serde_json::{json, Value};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bellande_limit", about = "Bellande Limit Tool")]
struct Opt {
    #[structopt(long, help = "Starting point coordinates as JSON-formatted list")]
    node0: String,

    #[structopt(long, help = "Target point coordinates as JSON-formatted list")]
    node1: String,

    #[structopt(long, help = "Environment dimensions as JSON-formatted list")]
    environment: String,

    #[structopt(long, help = "Step sizes for each dimension as JSON-formatted list")]
    size: String,

    #[structopt(long, help = "Goal coordinates as JSON-formatted list")]
    goal: String,

    #[structopt(long, help = "List of obstacles as JSON-formatted list")]
    obstacles: Option<String>,

    #[structopt(
        long,
        default_value = "50.0",
        help = "Search radius for obstacle detection"
    )]
    search_radius: f64,

    #[structopt(
        long,
        default_value = "20",
        help = "Number of sample points for obstacle detection"
    )]
    sample_points: i32,

    #[structopt(long, help = "Use local executable instead of API")]
    use_executable: bool,
}

pub async fn make_bellande_limit_request(
    node0: Value,
    node1: Value,
    environment: Value,
    size: Value,
    goal: Value,
    obstacles: Option<Value>,
    search_radius: f64,
    sample_points: i32,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = "https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Limit/bellande_limit";

    let payload = json!({
        "node0": node0,
        "node1": node1,
        "environment": environment,
        "size": size,
        "goal": goal,
        "obstacles": obstacles.unwrap_or(json!([])),
        "search_radius": search_radius,
        "sample_points": sample_points,
        "auth": {
            "authorization_key": "bellande_web_api_opensource"
        }
    });

    let response = client
        .post(url)
        .header("accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(response)
}

pub fn get_executable_path() -> PathBuf {
    if cfg!(target_os = "windows") {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Bellande_Limit.exe")
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Bellande_Limit")
    }
}

pub fn run_bellande_limit_executable(
    node0: &str,
    node1: &str,
    environment: &str,
    size: &str,
    goal: &str,
    obstacles: Option<&str>,
    search_radius: f64,
    sample_points: i32,
) -> Result<(), Box<dyn Error>> {
    let executable_path = get_executable_path();
    let passcode = "bellande_limit_executable_access_key";

    // Parse and validate inputs
    let node0_list: Value = serde_json::from_str(node0)?;
    let node1_list: Value = serde_json::from_str(node1)?;
    let environment_list: Value = serde_json::from_str(environment)?;
    let size_list: Value = serde_json::from_str(size)?;
    let goal_list: Value = serde_json::from_str(goal)?;
    let obstacles_list: Value = obstacles
        .map(|o| serde_json::from_str(o))
        .transpose()?
        .unwrap_or(json!([]));

    // Validate dimensions
    let dimensions = environment_list.as_array().unwrap().len();
    if let (Some(n0), Some(n1), Some(s), Some(g)) = (
        node0_list.as_array(),
        node1_list.as_array(),
        size_list.as_array(),
        goal_list.as_array(),
    ) {
        if n0.len() != dimensions
            || n1.len() != dimensions
            || s.len() != dimensions
            || g.len() != dimensions
        {
            return Err(format!("All coordinates must have {} dimensions", dimensions).into());
        }
    }

    // Validate obstacles
    if let Some(obstacles) = obstacles_list.as_array() {
        for obstacle in obstacles {
            if let (Some(pos), Some(dims)) = (
                obstacle.get("position").and_then(Value::as_array),
                obstacle.get("dimensions").and_then(Value::as_array),
            ) {
                if pos.len() != dimensions || dims.len() != dimensions {
                    return Err(
                        "Obstacle position and dimensions must match environment dimensions".into(),
                    );
                }
            }
        }
    }

    // Prepare and run command
    let mut command = Command::new(executable_path);
    command.args(&[
        passcode,
        node0,
        node1,
        environment,
        size,
        goal,
        &serde_json::to_string(&obstacles_list)?,
        &search_radius.to_string(),
        &sample_points.to_string(),
    ]);

    let output = command.output()?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    } else {
        Err(format!(
            "Process failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into())
    }
}
