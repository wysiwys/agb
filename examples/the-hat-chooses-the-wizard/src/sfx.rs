use agb::sound::mixer::{Mixer, SoundChannel};

mod music_data {
    // From the open game art page:
    //
    // USING THE LOOPED VERSION:
    // 1. Play the intro.
    // 2. When the intro reaches approximately 11.080 seconds, trigger the main loop and let the intro finish underneath it.
    // 3. Re-trigger the main loop every time it reaches 1 minute 26.080 seconds, and let the old instance finish underneath the new one.
    pub const INTRO_MUSIC: &[u8] =
        agb::include_wav!("sfx/Otto Halmén - Sylvan Waltz (loop intro).wav");
    pub const LOOP: &[u8] = agb::include_wav!("sfx/Otto Halmén - Sylvan Waltz (loop main).wav");

    // These are based on the instructions above and a frame rate of 59.73Hz
    pub const TRIGGER_MUSIC_POINT: i32 = 662;
    pub const LOOP_MUSIC: i32 = 5141;
}

mod effects {
    const WOOSH1: &[u8] = agb::include_wav!("sfx/woosh1.wav");
    const WOOSH2: &[u8] = agb::include_wav!("sfx/woosh2.wav");
    const WOOSH3: &[u8] = agb::include_wav!("sfx/woosh3.wav");

    pub const WHOOSHES: &[&[u8]] = &[WOOSH1, WOOSH2, WOOSH3];

    pub const CATCH: &[u8] = agb::include_wav!("sfx/catch.wav");

    pub const JUMP: &[u8] = agb::include_wav!("sfx/jump.wav");
    pub const LAND: &[u8] = agb::include_wav!("sfx/land.wav");

    pub const SLIME_JUMP: &[u8] = agb::include_wav!("sfx/slime-jump.wav");
    pub const SLIME_DEATH: &[u8] = agb::include_wav!("sfx/slime-death.wav");

    pub const SNAIL_EMERGE: &[u8] = agb::include_wav!("sfx/snail-emerge.wav");
    pub const SNAIL_RETREAT: &[u8] = agb::include_wav!("sfx/snail-retreat.wav");
    pub const SNAIL_HAT_BOUNCE: &[u8] = agb::include_wav!("sfx/snail-hat-bounce.wav");
    pub const SNAIL_DEATH: &[u8] = agb::include_wav!("sfx/snail-death.wav");
}

pub struct MusicBox {
    frame: i32,
}

impl MusicBox {
    pub fn new() -> Self {
        MusicBox { frame: 0 }
    }

    pub fn after_blank(&mut self, mixer: &mut Mixer) {
        if self.frame == 0 {
            // play the introduction
            mixer.play_sound(SoundChannel::new_high_priority(music_data::INTRO_MUSIC));
        } else if self.frame == music_data::TRIGGER_MUSIC_POINT
            || (self.frame - music_data::TRIGGER_MUSIC_POINT) % music_data::LOOP_MUSIC == 0
        {
            mixer.play_sound(SoundChannel::new_high_priority(music_data::LOOP));
        }

        self.frame += 1;
    }
}

pub struct SfxPlayer<'a, 'b> {
    mixer: &'a mut Mixer<'b>,
    frame: i32,
}

impl<'a, 'b> SfxPlayer<'a, 'b> {
    pub fn new(mixer: &'a mut Mixer<'b>, music_box: &MusicBox) -> Self {
        SfxPlayer {
            mixer,
            frame: music_box.frame,
        }
    }

    pub fn catch(&mut self) {
        self.mixer.play_sound(SoundChannel::new(effects::CATCH));
    }

    pub fn throw(&mut self) {
        self.play_random(effects::WHOOSHES);
    }

    pub fn jump(&mut self) {
        self.mixer.play_sound(SoundChannel::new(effects::JUMP));
    }

    pub fn slime_jump(&mut self) {
        self.mixer
            .play_sound(SoundChannel::new(effects::SLIME_JUMP));
    }

    pub fn slime_death(&mut self) {
        self.mixer
            .play_sound(SoundChannel::new(effects::SLIME_DEATH));
    }
    pub fn snail_emerge(&mut self) {
        self.mixer
            .play_sound(SoundChannel::new(effects::SNAIL_EMERGE));
    }

    pub fn snail_retreat(&mut self) {
        self.mixer
            .play_sound(SoundChannel::new(effects::SNAIL_RETREAT));
    }

    pub fn snail_hat_bounce(&mut self) {
        self.mixer
            .play_sound(SoundChannel::new(effects::SNAIL_HAT_BOUNCE));
    }

    pub fn snail_death(&mut self) {
        self.mixer
            .play_sound(SoundChannel::new(effects::SNAIL_DEATH));
    }

    pub fn land(&mut self) {
        self.mixer.play_sound(SoundChannel::new(effects::LAND));
    }

    fn play_random(&mut self, effect: &[&'static [u8]]) {
        self.mixer.play_sound(SoundChannel::new(
            effect[(self.frame as usize) % effect.len()],
        ));
    }
}
