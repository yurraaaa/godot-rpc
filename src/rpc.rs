use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient,
    activity::{Activity, ActivityType, Assets, Timestamps},
};
use godot::{classes::ProjectSettings, prelude::*};

const PROJECT_NAME_PATH: &str = "application/config/name";
const UPDATE_TIME: Duration = Duration::from_secs(30);
const ICON_URL: &str = "https://godotengine.org/assets/press/icon_color.png";

#[derive(GodotClass)]
#[class(init, singleton)]
pub struct DiscordRPC {
    client: Option<DiscordIpcClient>,
    app_id: String,
    details: String,

    started_at: i64,
    last_update: Option<Instant>,

    base: Base<Object>,
}

impl DiscordRPC {
    pub fn setup(&mut self, application_id: &str) {
        self.app_id = application_id.to_owned();

        let project_name: String = ProjectSettings::singleton()
            .get_setting(PROJECT_NAME_PATH)
            .to();

        let name = if project_name.is_empty() {
            "Unnamed Project"
        } else {
            &project_name
        };
        self.details = format!("In project: {name}");

        self.started_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        self.last_update = None;
        self.client = None;

        self.try_connect();
    }

    fn try_connect(&mut self) -> bool {
        if self.client.is_some() {
            return true;
        }

        if self.app_id.is_empty() {
            return false;
        }

        let mut client = DiscordIpcClient::new(&self.app_id);
        if client.connect().is_ok() {
            self.client = Some(client);
            true
        } else {
            self.client = None;
            false
        }
    }

    pub fn update(&mut self) {
        if let Some(last_update) = self.last_update
            && last_update.elapsed() < UPDATE_TIME
        {
            return;
        }

        self.last_update = Some(Instant::now());

        if !self.try_connect() {
            return;
        }

        if let Some(client) = &mut self.client {
            let activity = Activity::new()
                .name("Godot Engine")
                .details(&self.details)
                .assets(Assets::new().large_image(ICON_URL))
                .timestamps(Timestamps::new().start(self.started_at))
                .activity_type(ActivityType::Playing);

            if client.set_activity(activity).is_err() {
                self.client = None;
            }
        }
    }

    pub fn close(&mut self) {
        if let Some(mut client) = self.client.take() {
            let _ = client.close();
        }
    }
}
