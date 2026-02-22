use macroquad::prelude::*;

mod player;
use player::Player;

#[macroquad::main("RustPac - Jetpac Clone")]
async fn main() {
    // Initialisation du joueur au centre de l'écran
    let mut player = Player::new(screen_width() / 2.0 - 12.0, screen_height() / 2.0 - 12.0);
    
    println!("RustPac - Contrôles: ← → pour bouger, ↑ ou ESPACE pour propulsion");

    loop {
        clear_background(BLACK);

        // Mise à jour
        player.update();

        // Dessin
        player.draw();
        
        // UI
        draw_text("RustPac - Étape 1", 10.0, 30.0, 24.0, WHITE);
        draw_text("← → : déplacement | ↑ ou ESPACE : propulsion | Échap : quitter", 10.0, 55.0, 16.0, GRAY);

        // Quitter avec Échap
        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
