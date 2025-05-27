use std::thread;
use std::time::Duration;
use std::io::{self, Write, Read}; // Kombinierte io-Importe
use rand::Rng; // Für den Neon-Flatter-Effekt

// Hilfsfunktion zum Drucken einer Zeile innerhalb des Rahmens, zentriert
fn print_boxed_line(content: &str, inner_width: usize, vertical_border: char) {
    let content_chars: Vec<char> = content.chars().collect();
    let content_actual_width = content_chars.len(); // tatsächliche Breite des Inhalts
    
    let padding_total = inner_width.saturating_sub(content_actual_width);
    let padding_left = padding_total / 2;
    let padding_right = padding_total - padding_left;

    print!("{}", vertical_border);
    for _ in 0..padding_left { print!(" "); }
    print!("{}", content);
    for _ in 0..padding_right { print!(" "); }
    println!("{}", vertical_border);
}


// main-Funktion: Richtet das Programm ein und führt die Hauptschleife aus
fn main() {
    // Deklariere veränderliche Variablen zum Speichern der Benutzereingaben
    let mut userinput1: i32;
    let mut userinput2: i32;

    // Zeige die Einführung des Taschenrechners an
    calc_intro();

    // Hauptschleife, um kontinuierlich Zahlen zu erhalten und zu berechnen
    loop {
        userinput1 = get_user_number("1"); // Hole die erste Zahl
        userinput2 = get_user_number("2"); // Hole die zweite Zahl
        calc(userinput1, userinput2);    // Führe die Berechnung durch und zeige das Ergebnis an
    }
}

// Funktion, um eine gültige Ganzzahleingabe vom Benutzer zu erhalten
fn get_user_number(input_number_label: &str) -> i32 {
    loop {
        // Fordere den Benutzer zur Eingabe auf
        println!("Bitte geben Sie Wert {} ein:", input_number_label); // Nachricht angepasst
        io::stdout().flush().unwrap_or_else(|e| eprintln!("Fehler beim Leeren von stdout: {}", e)); // Stelle sicher, dass die Aufforderung angezeigt wird

        let mut user_input_str = String::new();
        // Lese die Zeile von der Standardeingabe
        if io::stdin().read_line(&mut user_input_str).is_err() {
            println!("Fehler beim Lesen der Zeile. Bitte versuchen Sie es erneut.");
            continue;
        }

        let trimmed_input = user_input_str.trim(); // Entferne Leerzeichen

        // Versuche, die Eingabezeichenkette als i32-Ganzzahl zu parsen
        match trimmed_input.parse::<i32>() {
            Ok(number) => {
                return number; // Gebe die gültige Zahl zurück
            }
            Err(_) => {
                // Behandle ungültige Eingaben
                println!("Bitte geben Sie eine gültige Zahl ein."); // Fehlermeldung höflicher gestaltet
                continue; // Setze die Schleife fort, um erneut nach Eingaben zu fragen
            }
        }
    }
}

// Funktion zur Durchführung der Berechnung und Anzeige der Ergebnisse mit einer "Denk"-Animation
fn calc(value1: i32, value2: i32) {
    let sleep_time = Duration::from_millis(1000); // Dauer für den Schlafmodus
    let value3: i32 = value1 + value2;            // Berechne die Summe

    // Gebe die Eingabewerte aus
    println!("Wert 1: {}", value1);
    println!("Wert 2: {}", value2);

    // "Denk"-Animation
    print!("Denke nach");
    io::stdout().flush().unwrap_or_else(|e| eprintln!("Fehler beim Leeren von stdout: {}", e)); 
    for _ in 0..3 {
        thread::sleep(sleep_time); // Pausiere für eine Sekunde
        print!(".");
        io::stdout().flush().unwrap_or_else(|e| eprintln!("Fehler beim Leeren von stdout: {}", e)); 
    }

    println!(); // Neue Zeile nach "Denke nach..."
    println!("{} + {} = {}", value1, value2, value3); // Gebe das Berechnungsergebnis aus

    // Fordere auf, fortzufahren, bevor der Bildschirm geleert wird
    println!("\nDrücken Sie eine beliebige Taste, um eine weitere Berechnung durchzuführen...");
    let mut stdin_pause = io::stdin();
    let mut buffer_pause = [0; 1];
    let _ = stdin_pause.read(&mut buffer_pause); // Warte auf einen Tastendruck

    clear_screen(); // Leere den Bildschirm für die nächste Berechnung
}

// Funktion zum Leeren des Terminalbildschirms mit ANSI-Escape-Codes
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H"); // ANSI-Escape-Code zum Leeren des Bildschirms und Bewegen des Cursors nach oben links
    io::stdout().flush().unwrap_or_else(|e| eprintln!("Fehler beim Leeren von stdout: {}", e));
}

// Funktion zur Anzeige der Einführung des Taschenrechners
fn calc_intro() {
    let title_lines = [
        " ██████╗ █████╗ ██╗      ██████╗██╗  ██╗██╗      █████╗ ████████╗ ██████╗ ██████╗ ",
        "██╔════╝██╔══██╗██║      ██╔════╝██║  ██║██║     ██╔══██╗╚══██╔══╝██╔═══██╗██╔══██╗",
        "██║     ███████║██║      ██║     ██║  ██║██║     ███████║   ██║   ██║   ██║██████╔╝",
        "██║     ██╔══██║██║      ██║     ██║  ██║██║     ██╔══██║   ██║   ██║   ██║██╔══██╗",
        "╚██████╗██║  ██║███████╗╚██████╗╚██████╔╝███████╗██║  ██║   ██║   ╚██████╔╝██║  ██║",
        " ╚═════╝╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝"
    ];
    let box_inner_width = title_lines[0].chars().count(); // Breite des Inhalts im Kasten
    let title_height = title_lines.len();

    let calculator_icon = [
        "   _____________________",
        "  |  _________________  |",
        "  | | JO            0. | |",
        "  | |_________________| |",
        "  |  ___ ___ ___   ___  |",
        "  | | 7 | 8 | 9 | | + | |",
        "  | |___|___|___| |___| |",
        "  | | 4 | 5 | 6 | | - | |",
        "  | |___|___|___| |___| |",
        "  | | 1 | 2 | 3 | | x | |",
        "  | |___|___|___| |___| |",
        "  | | . | 0 | = | | / | |",
        "  | |___|___|___| |___| |",
        "  |_____________________|",
    ];
    let icon_height = calculator_icon.len();

    let border_char_h = '─';
    let border_char_v = '│';
    let border_char_tl = '┌';
    let border_char_tr = '┐';
    let border_char_bl = '└';
    let border_char_br = '┘';

    // Definition der Höhen der einzelnen Sektionen im Kasten
    let top_blank_lines = 1;
    let underline_height = 1;
    let mid_blank_lines = 1; // Leerzeile zwischen Unterstrich und Icon
    let bottom_blank_lines = 1; // Leerzeile nach dem Icon

    // Gesamtzahl der inneren Zeilen (ohne oberen/unteren Rahmen)
    let num_inner_lines = top_blank_lines + title_height + underline_height + mid_blank_lines + icon_height + bottom_blank_lines;

    let horizontal_border_line = String::from(border_char_h).repeat(box_inner_width);

    // 1. Rahmenanimation (Kasten wird "gezeichnet")
    clear_screen();
    println!("{}{}{}", border_char_tl, horizontal_border_line, border_char_tr);
    io::stdout().flush().unwrap_or_else(|e| eprintln!("Fehler: {}", e));
    
    for i in 0..num_inner_lines {
        print_boxed_line("", box_inner_width, border_char_v);
        io::stdout().flush().unwrap_or_else(|e| eprintln!("Fehler: {}", e));
        
        let is_top_blank = i < top_blank_lines;
        let is_title_area = i >= top_blank_lines && i < top_blank_lines + title_height;
        let is_icon_area = i >= top_blank_lines + title_height + underline_height + mid_blank_lines && i < top_blank_lines + title_height + underline_height + mid_blank_lines + icon_height;

        if is_title_area || is_icon_area {
            thread::sleep(Duration::from_millis(20)); // Etwas langsamer für "Inhaltsbereiche"
        } else {
            thread::sleep(Duration::from_millis(10)); // Schneller für leere Bereiche/Padding
        }
    }
    println!("{}{}{}", border_char_bl, horizontal_border_line, border_char_br);
    io::stdout().flush().unwrap_or_else(|e| eprintln!("Fehler: {}", e));
    thread::sleep(Duration::from_millis(200)); // Kurze Pause nach Rahmenzeichnung

    // 2. Titelaufbau-Animation
    for lines_to_show_idx in 0..title_lines.len() { 
        clear_screen();
        println!("{}{}{}", border_char_tl, horizontal_border_line, border_char_tr);

        for _ in 0..top_blank_lines { print_boxed_line("", box_inner_width, border_char_v); }

        for j in 0..=lines_to_show_idx { print_boxed_line(title_lines[j], box_inner_width, border_char_v); }
        for _ in (lines_to_show_idx + 1)..title_lines.len() { print_boxed_line("", box_inner_width, border_char_v); } // Rest des Titelbereichs leer

        let lines_after_title_area = underline_height + mid_blank_lines + icon_height + bottom_blank_lines;
        for _ in 0..lines_after_title_area { print_boxed_line("", box_inner_width, border_char_v); }
        
        println!("{}{}{}", border_char_bl, horizontal_border_line, border_char_br);
        io::stdout().flush().unwrap_or_else(|e| eprintln!("Fehler: {}", e));
        thread::sleep(Duration::from_millis(200));
    }
    
    // 3. Flutter-Animation (Neon-Effekt)
    let mut rng = rand::thread_rng();
    for _cycle in 0..25 { // Anzahl der Flatter-Zyklen
        clear_screen();
        println!("{}{}{}", border_char_tl, horizontal_border_line, border_char_tr);
        for _ in 0..top_blank_lines { print_boxed_line("", box_inner_width, border_char_v); }

        for title_line_content in &title_lines {
            let mut flutter_display_line = String::new();
            for original_char in title_line_content.chars() {
                if original_char != ' ' { // Nur Nicht-Leerzeichen flackern lassen
                    if rng.gen_bool(0.8) { // 80% Wahrscheinlichkeit, das Originalzeichen anzuzeigen
                        flutter_display_line.push(original_char);
                    } else { // 20% Wahrscheinlichkeit, ein Leerzeichen anzuzeigen (flackert aus)
                        flutter_display_line.push(' ');
                    }
                } else {
                    flutter_display_line.push(' '); // Leerzeichen bleiben Leerzeichen
                }
            }
            print_boxed_line(&flutter_display_line, box_inner_width, border_char_v);
        }
        
        let lines_after_title_area = underline_height + mid_blank_lines + icon_height + bottom_blank_lines;
        for _ in 0..lines_after_title_area { print_boxed_line("", box_inner_width, border_char_v); }

        println!("{}{}{}", border_char_bl, horizontal_border_line, border_char_br);
        io::stdout().flush().unwrap_or_else(|e| eprintln!("Fehler: {}", e));
        thread::sleep(Duration::from_millis(120)); 
    }

    // 4. Finale stabile Version des Titels mit animiertem Unterstrich und Icon
    clear_screen();
    println!("{}{}{}", border_char_tl, horizontal_border_line, border_char_tr);
    for _ in 0..top_blank_lines { print_boxed_line("", box_inner_width, border_char_v); }
    for line in &title_lines { print_boxed_line(line, box_inner_width, border_char_v); }
    
    // Unterstrich animiert zeichnen
    let underline_content = String::from(border_char_h).repeat(box_inner_width);
    print!("{}", border_char_v);
    io::stdout().flush().unwrap_or_else(|e| eprintln!("Fehler: {}", e));
    for u_char in underline_content.chars() {
        print!("{}", u_char);
        io::stdout().flush().unwrap_or_else(|e| eprintln!("Fehler: {}", e));
        thread::sleep(Duration::from_millis(4)); // Sehr schnelle Animation für den Unterstrich
    }
    println!("{}", border_char_v);
    
    for _ in 0..mid_blank_lines { print_boxed_line("", box_inner_width, border_char_v); }

    // Taschenrechner-Icon
    for icon_l in &calculator_icon { print_boxed_line(icon_l, box_inner_width, border_char_v); }
    
    for _ in 0..bottom_blank_lines { print_boxed_line("", box_inner_width, border_char_v); }
    println!("{}{}{}", border_char_bl, horizontal_border_line, border_char_br);

    println!("\n\nDrücken Sie eine beliebige Taste, um fortzufahren...");

    let mut stdin = io::stdin();
    let mut buffer = [0; 1]; 
    if stdin.read(&mut buffer).is_err() {
        eprintln!("Fehler beim Lesen der Eingabe zum Fortfahren.");
    }
    clear_screen(); 
}

