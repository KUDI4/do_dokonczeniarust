use dioxus::prelude::*;

fn main() {
    // Zwykłe launch automatycznie wykryje platformę webową
    launch(app);
}

fn app() -> Element {
    // Tworzymy stan dla nazwy użytkownika
    let mut username = use_signal(|| "Trader_Pro".to_string());

    rsx! {

        div { class: "container",
            // LEWA KOLUMNA (Twój boczny pasek)
            section { class: "bok",
                div { class: "bok1",
                    button { "1" }
                }
                div { class: "bok2",
                    button { "2" }
                }
                div { class: "bok3",
                    button { "3" }
                }
            }

            // PRAWA STRONA
            div { class: "prawa-strona",

                // NAGŁÓWEK Z PROFILEM
                header { class: "nag",
                    h1 { "Trading Journal" }

                    // Sekcja profilu użytkownika
                    div { class: "user-profile",
                        div { class: "user-info",
                            span { class: "username", "{username}" }
                            span { class: "user-role", "Premium" }
                        }
                        div { class: "avatar",
                            img { src: "https://liara.run", alt: "Avatar" }
                        }

                        // Rozwijane menu z przyciskiem zmiany konta
                        div { class: "profile-dropdown",
                            button {
                                class: "dropdown-item",
                                onclick: move |_| {
                                    if username.read().as_str() == "Trader_Pro" {
                                        username.set("Rekrut_Z_Rusta".to_string());
                                    } else {
                                        username.set("Trader_Pro".to_string());
                                    }
                                },
                                "🔄 Zmień konto"
                            }
                            button { class: "dropdown-item logout", "❌ Wyloguj" }
                        }
                    }
                }

                // ZAKŁADKI
                section { class: "zakladki",
                    div { class: "zakladka1",
                        button { "1" }
                    }
                    div { class: "zakladka2",
                        button { "2" }
                    }
                    div { class: "zakladka3",
                        button { "3" }
                    }
                    div { class: "zakladka4",
                        button { "4" }
                    }
                    div { class: "zakladka5",
                        button { "5" }
                    }
                }

                // GŁÓWNA TREŚĆ
                main { class: "main",
                    div { class: "notatki",
                        h1 { "TU BEDA NOTATKI TRADINGOWE" }
                    }
                }
            }
        }
    }
}