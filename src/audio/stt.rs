use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct LocalTranscriber {
    ctx: WhisperContext,
}

impl LocalTranscriber {
    pub fn new(model_path: &str) -> Result<Self, String> {
        // 1. Charger le modèle depuis le disque
        let ctx = WhisperContext::new_with_params(model_path, WhisperContextParameters::default())
            .map_err(|e| format!("Impossible de charger le modèle Whisper : {}", e))?;

        Ok(Self { ctx })
    }

    pub fn transcribe(&mut self, audio_data: &[f32]) -> Result<String, String> {
        // 2. Configurer les paramètres de reconnaissance
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

        // Optimisations pour la vitesse
        params.set_n_threads(4); // Utilise 4 cœurs CPU
        params.set_translate(false);
        params.set_language(Some("en")); // Force l'anglais pour aller plus vite (ou "fr")
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        // 3. Lancer le traitement (C'est bloquant, donc à mettre dans un thread/task !)
        // Create a state
        let mut state = self.ctx.create_state().map_err(|e| e.to_string())?;

        state
            .full(params, audio_data)
            .map_err(|e| format!("Erreur pendant la transcription : {}", e))?;

        // 4. Récupérer le texte
        let num_segments = state.full_n_segments();
        let mut text = String::new();

        // for i in 0..num_segments {
        //     if let Ok(segment) = state.full_get_segment_text(i) {
        //         text.push_str(&segment);
        //         text.push(' ');
        //     }
        // }

        Ok(text.trim().to_string())
    }
}
