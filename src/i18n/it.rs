//! Italian translations.

use std::collections::HashMap;

pub fn translations() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Navigation
    map.insert("nav.home", "Home");
    map.insert("nav.events", "Programma");
    map.insert("nav.rsvp", "RSVP");
    map.insert("nav.admin", "Admin");

    // Home page
    map.insert("home.title", "Ci sposiamo!");
    map.insert("home.subtitle", "SI SPOSANO");
    map.insert("home.welcome", "Benvenuto");
    map.insert(
        "home.intro_p1",
        "Siamo entusiasti di celebrare il nostro giorno speciale con voi!",
    );
    map.insert("home.intro_p2", "Esplora il nostro sito web per trovare tutte le informazioni di cui hai bisogno sugli eventi, le location e come confermare la presenza. Non vediamo l'ora di vederti lì!");
    map.insert("home.sardinia_title", "Sardegna");
    map.insert(
        "home.sardinia_desc",
        "Unisciti a noi per festeggiare il nostro matrimonio in Sardegna!",
    );
    map.insert("home.tunisia_title", "Tunisia");
    map.insert(
        "home.tunisia_desc",
        "Festeggia con noi nel cuore del Nord Africa con l'ospitalità tunisina",
    );
    map.insert("home.gift_message", "Celebriamo Insieme");
    map.insert(
        "home.contribution_text",
        "La nostra festa non sarebbe completa senza di voi. Saremmo onorati di avervi al nostro fianco per celebrare questo momento insieme.",
    );
    map.insert(
        "home.rsvp_instruction",
        "Vi preghiamo di comunicarci la vostra partecipazione compilando il modulo RSVP.",
    );
    map.insert("home.our_love", "C'era una Volta...");
    map.insert("home.for_gardens", "");
    map.insert("home.and_each", "nella Cité Universitaire");
    map.insert("home.other", "di Parigi");
    map.insert(
        "home.couple_story",
        "Era il 2017. Eravamo entrambi appena arrivati a Parigi; Mouna stava finendo i suoi studi e Mauro stava iniziando il suo dottorato. Tutto è iniziato con una cena, poi un'amicizia, una complicità. Quello che doveva essere un semplice incontro ha segnato l'inizio della nostra storia.",
    );
    map.insert("home.see_you_there", "Ci vediamo lì!");

    // Events page
    map.insert(
        "events.subtitle_single",
        "Unisciti a noi per celebrare il nostro amore",
    );
    map.insert(
        "events.subtitle_multiple",
        "Unisciti a noi per celebrare il nostro amore attraverso bellissime destinazioni",
    );
    map.insert("events.title", "Dettagli dell'evento");
    map.insert("events.sardinia", "Sardegna, Italia");
    map.insert("events.tunisia", "Tunisia");
    map.insert("events.nice", "Nizza, Francia");
    map.insert("events.schedule", "Programma");
    map.insert("events.venue", "Luogo");
    map.insert("events.accommodation", "Alloggio");
    map.insert("events.travel", "Informazioni di viaggio");
    map.insert("events.view_on_maps", "Visualizza su Google Maps");

    // Event content placeholders
    map.insert("events.date_sardinia", "19 Settembre 2026");
    map.insert("events.sort_date_sardinia", "2026-09-19");
    map.insert("events.schedule_sardinia", "Cerimonia alle 18:00");
    map.insert("events.venue_sardinia_name", "Sa Mola Hotel Ristorante");
    map.insert(
        "events.venue_sardinia_link",
        "https://maps.app.goo.gl/yNLukc3C9V6bPL4DA",
    );
    map.insert(
        "events.accommodation_sardinia",
        "• Hotel consigliati a <a href='https://www.google.com/maps/place/Oristano,+OR,+Italy' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Oristano</a> / <a href='https://www.google.com/maps/place/Cabras,+OR,+Italy' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Cabras</a>",
    );
    map.insert(
        "events.travel_sardinia",
        "• L'aeroporto di <a href='https://maps.app.goo.gl/uvofAX2NkqLeoi2D7' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Cagliari</a> (CA) è il più vicino<br/>• Si consiglia di noleggiare un'auto",
    );
    map.insert("events.date_tunisia", "27 Giugno 2026");
    map.insert("events.sort_date_tunisia", "2026-06-27");
    map.insert("events.schedule_tunisia", "Inizio alle 21:00");
    map.insert("events.venue_tunisia_name", "Espace La Vallée, Monastir");
    map.insert(
        "events.venue_tunisia_link",
        "https://maps.app.goo.gl/Y4dCfdekMGiWvMFX6",
    );
    map.insert("events.accommodation_tunisia", "• Hotel a Monastir<br/>• Gli hotel possono essere prenotati anche nella <a href='https://www.google.com/maps/d/edit?mid=1saWGZmjkgOkyQZxfyFeMldJy3JWWvg8&usp=sharing' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>zona turistica di Monastir</a>");
    map.insert(
        "events.travel_tunisia",
        "• Aeroporto più vicino: <a href='https://maps.app.goo.gl/YyvPgoUmRDPqmzgy8' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Monastir</a><br/>• Raccomandiamo di evitare la compagnia aerea Tunisair<br/>• L'acqua di rubinetto non è potabile in Tunisia",
    );
    map.insert("events.date_nice", "8 Aprile 2026");
    map.insert("events.sort_date_nice", "2026-04-08");
    map.insert(
        "events.schedule_nice",
        "Cerimonia alle 11:00, Pranzo a seguire",
    );
    map.insert("events.venue_nice_name", "Municipio di Nizza");
    map.insert(
        "events.venue_nice_link",
        "https://maps.app.goo.gl/D9hQbstQqHWxa1m49",
    );
    map.insert("events.accommodation_nice", "• Hotel a Nizza<br/>• Privilegiate un alloggio lungo le linee del tram 2 e 3, preferibilmente vicino alla spiaggia");
    map.insert(
        "events.travel_nice",
        "• L'<a href='https://maps.app.goo.gl/8KRRidQakgL2C97t8' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>aeroporto di Nizza Costa Azzurra</a> serve la zona",
    );

    // RSVP
    map.insert("rsvp.title", "RSVP");
    map.insert("rsvp.subtitle", "Facci sapere se puoi unirti a noi!");
    map.insert("rsvp.lookup", "Trova il tuo invito");
    map.insert("rsvp.name", "Il tuo nome");
    map.insert("rsvp.code", "Codice invito");
    map.insert("rsvp.code_placeholder", "ABC123");
    map.insert(
        "rsvp.code_help",
        "Inserisci il codice invito dalla tua carta di invito",
    );
    map.insert("rsvp.email", "Email");
    map.insert("rsvp.find", "Trova RSVP");
    map.insert("rsvp.attending", "Parteciperai?");
    map.insert("rsvp.yes", "Sì");
    map.insert("rsvp.no", "No");
    map.insert("rsvp.guests", "Numero di ospiti");
    map.insert("rsvp.dietary", "Restrizioni dietetiche");
    map.insert(
        "rsvp.guest_list_title",
        "Lista degli ospiti e preferenze alimentari",
    );
    map.insert("rsvp.guest_list_description", "Gestisci la lista degli ospiti e le restrizioni alimentari qui. Potrai quindi selezionare quali ospiti parteciperanno a ciascuna location qui sotto.");
    map.insert("rsvp.vegetarian", "Vegetariano");
    map.insert("rsvp.vegan", "Vegano");
    map.insert("rsvp.halal", "Halal");
    map.insert("rsvp.no_pork", "Senza maiale");
    map.insert("rsvp.other", "Altro (Allergie, ecc.)");
    map.insert(
        "rsvp.other_dietary",
        "Altre restrizioni alimentari (specificare numero e tipo)",
    );
    map.insert("rsvp.dietary_number", "Ospiti");
    map.insert(
        "rsvp.dietary_placeholder",
        "es., Senza glutine, Senza frutti di mare, Intollerante al lattosio",
    );
    map.insert(
        "rsvp.dietary_help",
        "Aggiungi le restrizioni dietetiche con il numero di persone per ciascuna",
    );
    map.insert("rsvp.dietary_remaining", "rimanenti");
    map.insert("rsvp.notes", "Note aggiuntive");
    map.insert("rsvp.submit", "Invia RSVP");
    map.insert("rsvp.update", "Aggiorna RSVP");
    map.insert("rsvp.form_title_new", "Completa il tuo RSVP");
    map.insert("rsvp.form_title_update", "Aggiorna il tuo RSVP");
    map.insert("rsvp.welcome", "Benvenuto");
    map.insert("rsvp.both_events", "Sei invitato a entrambi gli eventi. Si prega di completare il RSVP per ciascuna località separatamente.");
    map.insert("rsvp.party_size", "Dimensione gruppo");
    map.insert("rsvp.guests_label", "ospite/i");
    map.insert("rsvp.success", "Grazie! Il tuo RSVP è stato salvato.");
    map.insert("rsvp.success_thank_you", "Grazie per la tua risposta!");
    map.insert(
        "rsvp.success_refresh",
        "La pagina si ricaricherà tra 5 secondi.",
    );
    map.insert("rsvp.error", "Qualcosa è andato storto. Riprova.");
    map.insert("rsvp.error_code_required", "Inserisci il tuo codice invito");
    map.insert("rsvp.error_loading", "Errore nel caricamento del RSVP");
    map.insert(
        "rsvp.error_code_invalid",
        "Codice invito non trovato. Controlla il tuo codice e riprova.",
    );
    map.insert(
        "rsvp.error_network",
        "Errore di rete. Controlla la tua connessione e riprova.",
    );
    map.insert(
        "rsvp.error_generic",
        "Si è verificato un errore. Riprova più tardi.",
    );
    map.insert(
        "rsvp.error_empty_names",
        "Compila i nomi di tutti gli ospiti prima di inviare.",
    );
    map.insert(
        "rsvp.error_no_locations",
        "Seleziona almeno una location per ogni ospite.",
    );
    map.insert(
        "rsvp.not_found",
        "Ospite non trovato. Verifica le tue informazioni.",
    );
    map.insert("rsvp.invitees_title", "I Tuoi Ospiti");
    map.insert("rsvp.invitee_name", "Nome dell'Ospite");
    map.insert("rsvp.add_invitee", "Aggiungi Ospite");
    map.insert("rsvp.delete_invitee", "Rimuovi Ospite");
    map.insert("rsvp.gluten_free", "Senza Glutine");
    map.insert("rsvp.guest_list", "Lista degli ospiti");
    map.insert(
        "rsvp.guest_list_help",
        "Aggiungi tutti gli ospiti del tuo gruppo e le loro preferenze alimentari",
    );
    map.insert("rsvp.add_another_guest", "+ Aggiungi un altro ospite");
    map.insert(
        "rsvp.notes_help",
        "Richieste speciali, restrizioni alimentari o messaggi per noi?",
    );
    map.insert("rsvp.notes_placeholder", "Richieste speciali o messaggi?");
    map.insert("rsvp.attending_label", "Partecipa a:");
    map.insert("rsvp.age_category", "Categoria di età:");
    map.insert("rsvp.adult", "Adulto");
    map.insert("rsvp.child_under_3", "< 3 anni");
    map.insert("rsvp.child_under_10", "< 10 anni");
    map.insert("rsvp.dietary_restrictions_label", "Restrizioni alimentari:");
    map.insert("rsvp.guest_not_found", "Ospite non trovato");

    // Admin
    map.insert("admin.title", "Dashboard Admin");
    map.insert("admin.login", "Accedi");
    map.insert("admin.logout", "Esci");
    map.insert("admin.dashboard", "Dashboard");
    map.insert("admin.guests", "Ospiti");
    map.insert("admin.rsvps", "RSVPs");

    // Common
    map.insert("common.loading", "Caricamento...");
    map.insert("common.save", "Salva");
    map.insert("common.saving", "Salvataggio...");
    map.insert("common.cancel", "Annulla");
    map.insert("common.delete", "Elimina");
    map.insert("common.edit", "Modifica");
    map.insert("common.add", "Aggiungi");
    map.insert("common.search", "Cerca");
    map.insert("common.filter", "Filtra");
    map.insert("common.export", "Esporta");
    map.insert("common.back", "Indietro");
    map.insert("common.next", "Avanti");
    map.insert("common.previous", "Precedente");

    // Footer
    map.insert(
        "footer.copyright",
        "© 2026 - Fatto con ❤️ per il nostro giorno speciale",
    );

    map
}
