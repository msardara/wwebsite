use crate::types::Language;
use std::collections::HashMap;

pub struct Translations {
    language: Language,
}

impl Translations {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    pub fn t(&self, key: &str) -> String {
        let translations = match self.language {
            Language::English => self.en_translations(),
            Language::French => self.fr_translations(),
            Language::Italian => self.it_translations(),
        };

        translations
            .get(key)
            .map(|s| s.to_string())
            .unwrap_or_else(|| key.to_string())
    }

    fn en_translations(&self) -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();

        // Navigation
        map.insert("nav.home", "Home");
        map.insert("nav.events", "Events");
        map.insert("nav.gallery", "Gallery");
        map.insert("nav.rsvp", "RSVP");
        map.insert("nav.admin", "Admin");

        // Home page
        map.insert("home.title", "We're Getting Married!");
        map.insert("home.subtitle", "Join us in celebrating our special day");
        map.insert("home.welcome", "Welcome");
        map.insert(
            "home.intro_p1",
            "We are so excited to celebrate our special day with you!",
        );
        map.insert("home.intro_p2", "Please explore our website to find all the information you need about the events, venues, and how to RSVP. We can't wait to see you there!");
        map.insert("home.sardinia_title", "Sardinia");
        map.insert(
            "home.sardinia_desc",
            "Join us to celebrate our wedding in Sardinia!",
        );
        map.insert("home.tunisia_title", "Tunisia");
        map.insert(
            "home.tunisia_desc",
            "Celebrate with us in the heart of North Africa with Tunisian hospitality",
        );

        // Events page
        map.insert("events.title", "Event Details");
        map.insert("events.sardinia", "Sardinia, Italy");
        map.insert("events.tunisia", "Tunisia");
        map.insert("events.schedule", "Schedule");
        map.insert("events.venue", "Venue");
        map.insert("events.accommodation", "Accommodation");
        map.insert("events.travel", "Travel Information");
        map.insert("events.view_on_maps", "View on Google Maps");

        // Event content placeholders
        map.insert("events.schedule_sardinia", "Ceremony at 6:00 PM");
        map.insert("events.venue_sardinia_name", "Sa Mola Hotel Ristorante");
        map.insert(
            "events.venue_sardinia_link",
            "https://maps.app.goo.gl/yNLukc3C9V6bPL4DA",
        );
        map.insert(
            "events.accommodation_sardinia",
            "Recommended hotels in Oristano / Cabras.",
        );
        map.insert(
            "events.travel_sardinia",
            "Cagliari Airport (CA) is the closest. We recommend renting a car.",
        );
        map.insert(
            "events.schedule_tunisia",
            "Ceremony at 6:00 PM, Reception to follow at 7:00 PM",
        );
        map.insert("events.venue_tunisia_name", "Monastir");
        map.insert(
            "events.venue_tunisia_link",
            "https://maps.app.goo.gl/qhCtxDfV1PYgczaG6",
        );
        map.insert("events.accommodation_tunisia", "Hotels in Monastir");
        map.insert(
            "events.travel_tunisia",
            "Monastir Habib Bourguiba International Airport serves the area",
        );

        // Gallery
        map.insert("gallery.title", "Photo Gallery");
        map.insert("gallery.empty", "No photos yet. Check back soon!");

        // RSVP
        map.insert("rsvp.title", "RSVP");
        map.insert("rsvp.subtitle", "Let us know if you can join us!");
        map.insert("rsvp.lookup", "Find Your Invitation");
        map.insert("rsvp.name", "Your Name");
        map.insert("rsvp.code", "Invitation Code");
        map.insert("rsvp.code_placeholder", "ABC123");
        map.insert(
            "rsvp.code_help",
            "Enter the invitation code from your invitation card",
        );
        map.insert("rsvp.email", "Email");
        map.insert("rsvp.find", "Find RSVP");
        map.insert("rsvp.attending", "Will you be attending?");
        map.insert("rsvp.yes", "Yes");
        map.insert("rsvp.no", "No");
        map.insert("rsvp.guests", "Number of Guests");
        map.insert("rsvp.dietary", "Dietary Restrictions");
        map.insert("rsvp.guest_list_title", "Guest List & Dietary Preferences");
        map.insert("rsvp.guest_list_description", "Manage your guest list and dietary restrictions here. You can then select which guests attend each location below.");
        map.insert("rsvp.vegetarian", "Vegetarian");
        map.insert("rsvp.vegan", "Vegan");
        map.insert("rsvp.halal", "Halal");
        map.insert("rsvp.no_pork", "No Pork");
        map.insert("rsvp.other", "Other (Allergies, etc.)");
        map.insert(
            "rsvp.other_dietary",
            "Other dietary restrictions (specify number and type)",
        );
        map.insert("rsvp.dietary_number", "Guests");
        map.insert(
            "rsvp.dietary_placeholder",
            "e.g., Gluten-free, No shellfish, Lactose intolerant",
        );
        map.insert(
            "rsvp.dietary_help",
            "Add dietary restrictions with the number of people for each",
        );
        map.insert("rsvp.dietary_remaining", "remaining");
        map.insert("rsvp.notes", "Additional Notes");
        map.insert("rsvp.submit", "Submit RSVP");
        map.insert("rsvp.update", "Update RSVP");
        map.insert("rsvp.form_title_new", "Complete Your RSVP");
        map.insert("rsvp.form_title_update", "Update Your RSVP");
        map.insert("rsvp.welcome", "Welcome");
        map.insert("rsvp.both_events", "You are invited to both events. Please complete your RSVP for each location separately.");
        map.insert("rsvp.party_size", "Party size");
        map.insert("rsvp.guests_label", "guest(s)");
        map.insert("rsvp.success", "Thank you! Your RSVP has been saved.");
        map.insert("rsvp.success_thank_you", "Thank you for your response!");
        map.insert("rsvp.error", "Something went wrong. Please try again.");
        map.insert(
            "rsvp.error_code_required",
            "Please enter your invitation code",
        );
        map.insert("rsvp.error_loading", "Error loading RSVP");
        map.insert(
            "rsvp.error_code_invalid",
            "Invitation code not found. Please check your code and try again.",
        );
        map.insert(
            "rsvp.error_network",
            "Network error. Please check your connection and try again.",
        );
        map.insert(
            "rsvp.error_generic",
            "An error occurred. Please try again later.",
        );
        map.insert(
            "rsvp.not_found",
            "Guest not found. Please check your information.",
        );
        map.insert("rsvp.invitees_title", "Your Guests");
        map.insert("rsvp.invitee_name", "Guest Name");
        map.insert("rsvp.add_invitee", "Add Guest");
        map.insert("rsvp.delete_invitee", "Remove Guest");
        map.insert("rsvp.gluten_free", "Gluten Free");

        // Admin
        map.insert("admin.title", "Admin Dashboard");
        map.insert("admin.login", "Login");
        map.insert("admin.logout", "Logout");
        map.insert("admin.dashboard", "Dashboard");
        map.insert("admin.guests", "Guests");
        map.insert("admin.rsvps", "RSVPs");
        map.insert("admin.content", "Content");
        map.insert("admin.photos", "Photos");
        map.insert("admin.config", "Configuration");

        // Common
        map.insert("common.loading", "Loading...");
        map.insert("common.save", "Save");
        map.insert("common.saving", "Saving...");
        map.insert("common.cancel", "Cancel");
        map.insert("common.delete", "Delete");
        map.insert("common.edit", "Edit");
        map.insert("common.add", "Add");
        map.insert("common.search", "Search");
        map.insert("common.filter", "Filter");
        map.insert("common.export", "Export");
        map.insert("common.back", "Back");
        map.insert("common.next", "Next");
        map.insert("common.previous", "Previous");

        // Error messages
        map.insert(
            "error.generic",
            "An unexpected error occurred. Please try again.",
        );
        map.insert(
            "error.network",
            "Network error. Please check your connection and try again.",
        );
        map.insert(
            "error.auth",
            "Authentication failed. Please check your credentials.",
        );
        map.insert("error.not_found", "The requested resource was not found.");
        map.insert("error.validation", "Please check your input and try again.");
        map.insert("error.server", "Server error. Please try again later.");
        map.insert("error.storage", "File upload error. Please try again.");
        map.insert(
            "error.session_expired",
            "Your session has expired. Please log in again.",
        );

        // Footer
        map.insert(
            "footer.copyright",
            "© 2026 - Made with ❤️ for our special day",
        );

        map
    }

    fn fr_translations(&self) -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();

        // Navigation
        map.insert("nav.home", "Accueil");
        map.insert("nav.events", "Événements");
        map.insert("nav.gallery", "Galerie");
        map.insert("nav.rsvp", "RSVP");
        map.insert("nav.admin", "Admin");

        // Home page
        map.insert("home.title", "Nous nous marions !");
        map.insert(
            "home.subtitle",
            "Rejoignez-nous pour célébrer notre jour spécial",
        );
        map.insert("home.welcome", "Bienvenue");
        map.insert(
            "home.intro_p1",
            "Nous sommes tellement excités de célébrer notre jour spécial avec vous!",
        );
        map.insert("home.intro_p2", "Veuillez explorer notre site Web pour trouver toutes les informations dont vous avez besoin sur les événements, les lieux et comment répondre. Nous avons hâte de vous y voir !");
        map.insert("home.sardinia_title", "Sardaigne");
        map.insert(
            "home.sardinia_desc",
            "Rejoignez-nous pour célébrer notre mariage en Sardaigne !",
        );
        map.insert("home.tunisia_title", "Tunisie");
        map.insert(
            "home.tunisia_desc",
            "Célébrez avec nous au cœur de l'Afrique du Nord avec l'hospitalité tunisienne",
        );

        // Events page
        map.insert("events.title", "Détails de l'événement");
        map.insert("events.sardinia", "Sardaigne, Italie");
        map.insert("events.tunisia", "Tunisie");
        map.insert("events.schedule", "Programme");
        map.insert("events.venue", "Lieu");
        map.insert("events.accommodation", "Hébergement");
        map.insert("events.travel", "Informations de voyage");
        map.insert("events.view_on_maps", "Voir sur Google Maps");

        // Event content placeholders
        map.insert("events.schedule_sardinia", "Cérémonie à 18h00");
        map.insert("events.venue_sardinia_name", "Sa Mola Hotel Ristorante");
        map.insert(
            "events.venue_sardinia_link",
            "https://maps.app.goo.gl/yNLukc3C9V6bPL4DA",
        );
        map.insert(
            "events.accommodation_sardinia",
            "Hôtels recommandés à Oristano / Cabras.",
        );
        map.insert("events.travel_sardinia", "L'aéroport de Cagliari (CA) est le plus proche. Nous recommandons de louer une voiture.");
        map.insert(
            "events.schedule_tunisia",
            "Cérémonie à 18h00, Réception à 19h00",
        );
        map.insert("events.venue_tunisia_name", "Monastir");
        map.insert(
            "events.venue_tunisia_link",
            "https://maps.app.goo.gl/qhCtxDfV1PYgczaG6",
        );
        map.insert("events.accommodation_tunisia", "Hôtels à Monastir");
        map.insert(
            "events.travel_tunisia",
            "L'aéroport international Monastir Habib Bourguiba dessert la région",
        );

        // Gallery
        map.insert("gallery.title", "Galerie photos");
        map.insert("gallery.empty", "Pas encore de photos. Revenez bientôt !");

        // RSVP
        map.insert("rsvp.title", "RSVP");
        map.insert(
            "rsvp.subtitle",
            "Faites-nous savoir si vous pouvez nous rejoindre !",
        );
        map.insert("rsvp.lookup", "Trouvez votre invitation");
        map.insert("rsvp.name", "Votre nom");
        map.insert("rsvp.code", "Code d'invitation");
        map.insert("rsvp.code_placeholder", "ABC123");
        map.insert(
            "rsvp.code_help",
            "Entrez le code d'invitation de votre carte d'invitation",
        );
        map.insert("rsvp.email", "Email");
        map.insert("rsvp.find", "Trouver RSVP");
        map.insert("rsvp.attending", "Serez-vous présent ?");
        map.insert("rsvp.yes", "Oui");
        map.insert("rsvp.no", "Non");
        map.insert("rsvp.guests", "Nombre d'invités");
        map.insert("rsvp.dietary", "Restrictions alimentaires");
        map.insert(
            "rsvp.guest_list_title",
            "Liste des invités et préférences alimentaires",
        );
        map.insert("rsvp.guest_list_description", "Gérez votre liste d'invités et leurs restrictions alimentaires ici. Vous pourrez ensuite sélectionner quels invités assisteront à chaque événement ci-dessous.");
        map.insert("rsvp.vegetarian", "Végétarien");
        map.insert("rsvp.vegan", "Végétalien");
        map.insert("rsvp.halal", "Halal");
        map.insert("rsvp.no_pork", "Sans porc");
        map.insert("rsvp.other", "Autre (Allergies, etc.)");
        map.insert(
            "rsvp.other_dietary",
            "Autres restrictions alimentaires (préciser nombre et type)",
        );
        map.insert("rsvp.dietary_number", "Invités");
        map.insert(
            "rsvp.dietary_placeholder",
            "ex., Sans gluten, Sans fruits de mer, Intolérant au lactose",
        );
        map.insert(
            "rsvp.dietary_help",
            "Ajoutez les restrictions alimentaires avec le nombre de personnes pour chacune",
        );
        map.insert("rsvp.dietary_remaining", "restant(s)");
        map.insert("rsvp.notes", "Notes supplémentaires");
        map.insert("rsvp.submit", "Soumettre RSVP");
        map.insert("rsvp.update", "Mettre à jour RSVP");
        map.insert("rsvp.form_title_new", "Complétez votre RSVP");
        map.insert("rsvp.form_title_update", "Mettre à jour votre RSVP");
        map.insert("rsvp.welcome", "Bienvenue");
        map.insert("rsvp.both_events", "Vous êtes invité aux deux événements. Veuillez compléter votre RSVP pour chaque lieu séparément.");
        map.insert("rsvp.party_size", "Taille du groupe");
        map.insert("rsvp.guests_label", "invité(s)");
        map.insert("rsvp.success", "Merci ! Votre RSVP a été enregistré.");
        map.insert("rsvp.success_thank_you", "Merci pour votre réponse !");
        map.insert(
            "rsvp.error",
            "Une erreur s'est produite. Veuillez réessayer.",
        );
        map.insert(
            "rsvp.error_code_required",
            "Veuillez entrer votre code d'invitation",
        );
        map.insert("rsvp.error_loading", "Erreur lors du chargement du RSVP");
        map.insert(
            "rsvp.error_code_invalid",
            "Code d'invitation introuvable. Veuillez vérifier votre code et réessayer.",
        );
        map.insert(
            "rsvp.error_network",
            "Erreur réseau. Veuillez vérifier votre connexion et réessayer.",
        );
        map.insert(
            "rsvp.error_generic",
            "Une erreur s'est produite. Veuillez réessayer plus tard.",
        );
        map.insert(
            "rsvp.not_found",
            "Invité non trouvé. Veuillez vérifier vos informations.",
        );
        map.insert("rsvp.invitees_title", "Vos Invités");
        map.insert("rsvp.invitee_name", "Nom de l'Invité");
        map.insert("rsvp.add_invitee", "Ajouter un Invité");
        map.insert("rsvp.delete_invitee", "Retirer l'Invité");
        map.insert("rsvp.gluten_free", "Sans Gluten");

        // Admin
        map.insert("admin.title", "Tableau de bord Admin");
        map.insert("admin.login", "Connexion");
        map.insert("admin.logout", "Déconnexion");
        map.insert("admin.dashboard", "Tableau de bord");
        map.insert("admin.guests", "Invités");
        map.insert("admin.rsvps", "RSVPs");
        map.insert("admin.content", "Contenu");
        map.insert("admin.photos", "Photos");
        map.insert("admin.config", "Configuration");

        // Common
        map.insert("common.loading", "Chargement...");
        map.insert("common.save", "Enregistrer");
        map.insert("common.saving", "Enregistrement...");
        map.insert("common.cancel", "Annuler");
        map.insert("common.delete", "Supprimer");
        map.insert("common.edit", "Modifier");
        map.insert("common.add", "Ajouter");
        map.insert("common.search", "Rechercher");
        map.insert("common.filter", "Filtrer");
        map.insert("common.export", "Exporter");
        map.insert("common.back", "Retour");
        map.insert("common.next", "Suivant");
        map.insert("common.previous", "Précédent");

        // Footer
        map.insert(
            "footer.copyright",
            "© 2026 - Fait avec ❤️ pour notre jour spécial",
        );

        map
    }

    fn it_translations(&self) -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();

        // Navigation
        map.insert("nav.home", "Home");
        map.insert("nav.events", "Eventi");
        map.insert("nav.gallery", "Galleria");
        map.insert("nav.rsvp", "RSVP");
        map.insert("nav.admin", "Admin");

        // Home page
        map.insert("home.title", "Ci sposiamo!");
        map.insert(
            "home.subtitle",
            "Unisciti a noi per celebrare il nostro giorno speciale",
        );
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

        // Events page
        map.insert("events.title", "Dettagli dell'evento");
        map.insert("events.sardinia", "Sardegna, Italia");
        map.insert("events.tunisia", "Tunisia");
        map.insert("events.schedule", "Programma");
        map.insert("events.venue", "Luogo");
        map.insert("events.accommodation", "Alloggio");
        map.insert("events.travel", "Informazioni di viaggio");
        map.insert("events.view_on_maps", "Visualizza su Google Maps");

        // Event content placeholders
        map.insert("events.schedule_sardinia", "Cerimonia alle 18:00");
        map.insert("events.venue_sardinia_name", "Sa Mola Hotel Ristorante");
        map.insert(
            "events.venue_sardinia_link",
            "https://maps.app.goo.gl/yNLukc3C9V6bPL4DA",
        );
        map.insert(
            "events.accommodation_sardinia",
            "Hotel consigliati nella zona di Oristano / Cabras.",
        );
        map.insert(
            "events.travel_sardinia",
            "L'aeroporto di Cagliari (CA) è il più vicino. Consigliamo di noleggiare un'auto.",
        );
        map.insert(
            "events.schedule_tunisia",
            "Cerimonia alle 18:00, Ricevimento alle 19:00",
        );
        map.insert("events.venue_tunisia_name", "Monastir");
        map.insert(
            "events.venue_tunisia_link",
            "https://maps.app.goo.gl/qhCtxDfV1PYgczaG6",
        );
        map.insert("events.accommodation_tunisia", "Hotel a Monastir");
        map.insert(
            "events.travel_tunisia",
            "L'aeroporto internazionale Monastir Habib Bourguiba serve la zona",
        );

        // Gallery
        map.insert("gallery.title", "Galleria fotografica");
        map.insert("gallery.empty", "Nessuna foto ancora. Torna presto!");

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
            "rsvp.not_found",
            "Ospite non trovato. Verifica le tue informazioni.",
        );
        map.insert("rsvp.invitees_title", "I Tuoi Ospiti");
        map.insert("rsvp.invitee_name", "Nome dell'Ospite");
        map.insert("rsvp.add_invitee", "Aggiungi Ospite");
        map.insert("rsvp.delete_invitee", "Rimuovi Ospite");
        map.insert("rsvp.gluten_free", "Senza Glutine");

        // Admin
        map.insert("admin.title", "Dashboard Admin");
        map.insert("admin.login", "Accedi");
        map.insert("admin.logout", "Esci");
        map.insert("admin.dashboard", "Dashboard");
        map.insert("admin.guests", "Ospiti");
        map.insert("admin.rsvps", "RSVPs");
        map.insert("admin.content", "Contenuto");
        map.insert("admin.photos", "Foto");
        map.insert("admin.config", "Configurazione");

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
}
