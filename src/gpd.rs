use std::cmp::min;

use godot::engine::{AudioStreamGeneratorPlayback, AudioStreamPlayer2D, AudioStreamPlayer3D, Node, ProjectSettings};
use godot::prelude::*;
use libpd_rs::convenience::{calculate_ticks, PdGlobal};

const FRAME_BUFFER_LENGTH: usize = 512;

#[derive(GodotClass)]
#[class(base=Node)]
struct GPD {
    #[base]
    base: Base<Node>,

    #[var]
    audio_generator: Option<Gd<AudioStreamGeneratorPlayback>>,

    #[export]
    player_node_path: GString,

    #[export]
    patch_search_root: GString,

    #[export]
    entry_patch: GString,

    should_play: bool,
    pd: PdGlobal,
}

#[allow(non_snake_case)]
#[godot_api]
impl INode for GPD {
    fn init(base: Base<Node>) -> Self {
        let pd = PdGlobal::init_and_configure(0, 1, 48100).unwrap();

        // Let's evaluate a pd patch.
        // We could have opened a `.pd` file also.
        // This patch would play a sine wave at 440hz.
        GPD {
            base: base,
            audio_generator: None,
            entry_patch: GString::from(""),
            patch_search_root: GString::from(""),
            should_play: false,
            pd: pd,
            player_node_path: GString::from(""),
        }
    }

    fn ready(&mut self) {
        let mut gd_path = String::from("res://");
        gd_path.push_str(&self.patch_search_root.to_string());

        let search_path = std::path::PathBuf::from(ProjectSettings::singleton().globalize_path(GString::from(gd_path)).to_string());

        if self.patch_search_root.is_empty() {
            godot_error!("No patch search root set!");
        } else {
            if let Err(issue) = self.pd.add_path_to_search_paths(&search_path) {
                godot_error!("Issue with configured search path @ {}: \n {}", self.patch_search_root, issue);
            }
        }

        if self.entry_patch.is_empty() {
            godot_error!("No entrry patch set!");
        } else {
            if let Err(issue) = self.pd.open_patch(search_path.join(&self.entry_patch.to_string())) {
                godot_error!("Issue with configured entry patch @ {}: \n {}", self.patch_search_root, issue);
            }
        }

        let possible_untyped_player_node = self.base.get_node_or_null(NodePath::from(self.player_node_path.clone()));

        if let Some(untyped_player_node) = possible_untyped_player_node {
            if let Ok(mut audio_player) = untyped_player_node.clone().try_cast::<AudioStreamPlayer>() {
                audio_player.set_autoplay(true); // TODO, make this initialization happen at first play, rather than startup...
                self.audio_generator = Some(audio_player.get_stream_playback().unwrap().cast());
            } else if let Ok(mut audio_player_3D) = untyped_player_node.clone().try_cast::<AudioStreamPlayer3D>() {
                audio_player_3D.set_autoplay(true);
                self.audio_generator = Some(audio_player_3D.get_stream_playback().unwrap().cast());
            } else if let Ok(mut audio_player_2D) = untyped_player_node.clone().try_cast::<AudioStreamPlayer2D>() {
                audio_player_2D.set_autoplay(true);
                self.audio_generator = Some(audio_player_2D.get_stream_playback().unwrap().cast());
            } else {
                godot_error!("Player node found @ path {} is not a AudioStreamPlayer(nor a 3D or 2d variation of)", self.player_node_path);
            }
        } else {
            godot_error!("No player node found @ path {}", self.player_node_path);
        }
    }

    fn process(&mut self, _delta: f64) {
        if self.should_play {
            let frame_length = min(self.audio_generator().get_frames_available(), FRAME_BUFFER_LENGTH as i32);
            let mut outdata: [f32; FRAME_BUFFER_LENGTH] = [0.0; FRAME_BUFFER_LENGTH];
            let mut indata: [f32; FRAME_BUFFER_LENGTH] = [0.0; FRAME_BUFFER_LENGTH];
            libpd_rs::process::process_float(calculate_ticks(1, frame_length), &mut indata, &mut outdata);

            for i in 0..frame_length {
                self.audio_generator().push_frame(Vector2 {
                    x: outdata[i as usize],
                    y: outdata[i as usize],
                });
            }
        }
    }
}

#[godot_api]
impl GPD {
    #[func]
    fn play(&mut self) {
        self.should_play = true;
        self.pd.activate_audio(true).unwrap();
    }

    #[func]
    fn stop(&mut self) {
        self.should_play = false;
        self.pd.activate_audio(false).unwrap();
    }

    fn audio_generator(&mut self) -> &mut AudioStreamGeneratorPlayback {
        self.audio_generator.as_deref_mut().unwrap()
    }
}
