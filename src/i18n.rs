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
        map.insert("nav.events", "Program");

        map.insert("nav.rsvp", "RSVP");
        map.insert("nav.admin", "Admin");

        // Home page
        map.insert("home.title", "We're Getting Married!");
        map.insert("home.subtitle", "ARE GETTING MARRIED");
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
        map.insert("home.gift_message", "Let's Celebrate Together");
        map.insert(
            "home.contribution_text",
            "Our joy wouldn't be complete without you. We'd be honored to have you celebrate this moment with us.",
        );
        map.insert(
            "home.rsvp_instruction",
            "Please let us know if you can join by filling out the RSVP.",
        );
        map.insert("home.our_love", "Once Upon a Time...");
        map.insert("home.for_gardens", "");
        map.insert("home.and_each", "at the Cité Universitaire");
        map.insert("home.other", "de Paris");
        map.insert(
            "home.couple_story",
            "The year was 2017. We had both just arrived in Paris; Mouna was finishing her studies and Mauro was starting his PhD. It all began with a dinner, then a friendship, a connection. What was meant to be a simple meeting marked the beginning of our story.",
        );
        map.insert("home.see_you_there", "See you there!");

        // Events page
        map.insert("events.subtitle_single", "Join us in celebrating our love");
        map.insert(
            "events.subtitle_multiple",
            "Join us in celebrating our love across beautiful destinations",
        );
        map.insert("events.title", "Event Details");
        map.insert("events.sardinia", "Sardinia, Italy");
        map.insert("events.tunisia", "Tunisia");
        map.insert("events.nice", "Nice, France");
        map.insert("events.schedule", "Schedule");
        map.insert("events.venue", "Venue");
        map.insert("events.accommodation", "Accommodation");
        map.insert("events.travel", "Travel Information");
        map.insert("events.view_on_maps", "View on Google Maps");

        // Event content placeholders
        map.insert("events.date_sardinia", "September 19, 2026");
        map.insert("events.sort_date_sardinia", "2026-09-19");
        map.insert("events.schedule_sardinia", "Ceremony at 6:00 PM");
        map.insert("events.venue_sardinia_name", "Sa Mola Hotel Ristorante");
        map.insert(
            "events.venue_sardinia_link",
            "https://maps.app.goo.gl/yNLukc3C9V6bPL4DA",
        );
        map.insert(
            "events.accommodation_sardinia",
            "• Recommended hotels in <a href='https://www.google.com/maps/place/Oristano,+OR,+Italy' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Oristano</a> / <a href='https://www.google.com/maps/place/Cabras,+OR,+Italy' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Cabras</a>",
        );
        map.insert(
            "events.travel_sardinia",
            "• Cagliari Airport (CA) is the closest<br/>• We recommend renting a car",
        );
        map.insert("events.date_tunisia", "June 27, 2026");
        map.insert("events.sort_date_tunisia", "2026-06-27");
        map.insert("events.schedule_tunisia", "Starts at 9:00 PM");
        map.insert("events.venue_tunisia_name", "Espace La Vallée, Monastir");
        map.insert(
            "events.venue_tunisia_link",
            "https://maps.app.goo.gl/Y4dCfdekMGiWvMFX6",
        );
        map.insert("events.accommodation_tunisia", "• Hotels in Monastir<br/>• Hotels can also be booked in the <a href='https://www.google.com/maps/d/edit?mid=1saWGZmjkgOkyQZxfyFeMldJy3JWWvg8&usp=sharing' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>tourist area of Monastir</a>");
        map.insert(
            "events.travel_tunisia",
            "• Closest airport: Monastir<br/>• We recommend avoiding Tunisair airlines<br/>• Tap water is not drinkable in Tunisia",
        );
        map.insert("events.date_nice", "April 8, 2026");
        map.insert("events.sort_date_nice", "2026-04-08");
        map.insert(
            "events.schedule_nice",
            "Ceremony at 11:00 AM, Lunch to follow",
        );
        map.insert("events.venue_nice_name", "Nice City Hall");
        map.insert(
            "events.venue_nice_link",
            "https://maps.app.goo.gl/D9hQbstQqHWxa1m49",
        );
        map.insert("events.accommodation_nice", "• Hotels in Nice<br/>• Prefer accommodation along tram lines 2 and 3, preferably near the beach");
        map.insert(
            "events.travel_nice",
            "• <a href='https://maps.app.goo.gl/8KRRidQakgL2C97t8' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Nice Côte d'Azur Airport</a> serves the area",
        );

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
        map.insert("rsvp.success_refresh", "The page will reload in 5 seconds.");
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
        map.insert("rsvp.guest_list", "Guest List");
        map.insert(
            "rsvp.guest_list_help",
            "Add all guests in your party and their dietary preferences",
        );
        map.insert("rsvp.add_another_guest", "+ Add Another Guest");
        map.insert(
            "rsvp.notes_help",
            "Any special requests, dietary restrictions, or messages for us?",
        );
        map.insert(
            "rsvp.notes_placeholder",
            "Any special requests or messages?",
        );
        map.insert("rsvp.attending_label", "Attending:");
        map.insert("rsvp.age_category", "Age Category:");
        map.insert("rsvp.adult", "Adult");
        map.insert("rsvp.child_under_3", "< 3 years");
        map.insert("rsvp.child_under_10", "< 10 years");
        map.insert("rsvp.dietary_restrictions_label", "Dietary Restrictions:");
        map.insert("rsvp.guest_not_found", "Guest not found");

        // Admin
        map.insert("admin.title", "Admin Dashboard");
        map.insert("admin.login", "Login");
        map.insert("admin.logout", "Logout");
        map.insert("admin.dashboard", "Dashboard");
        map.insert("admin.guests", "Guests");
        map.insert("admin.rsvps", "RSVPs");

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
        map.insert("nav.events", "Programme");

        map.insert("nav.rsvp", "RSVP");
        map.insert("nav.admin", "Admin");

        // Home page
        map.insert("home.title", "Nous nous marions !");
        map.insert("home.subtitle", "SE MARIENT");
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
        map.insert("home.gift_message", "Célébrons Ensemble");
        map.insert(
            "home.contribution_text",
            "Notre joie ne serait pas complète sans vous. Nous serions honorés de vous avoir à nos côtés pour célébrer ce moment avec nous.",
        );
        map.insert(
            "home.rsvp_instruction",
            "Merci de nous faire savoir si vous pourrez vous joindre à nous en remplissant le RSVP.",
        );
        map.insert("home.our_love", "Il Était une Fois...");
        map.insert("home.for_gardens", "");
        map.insert("home.and_each", "à la Cité Universitaire");
        map.insert("home.other", "de Paris");
        map.insert(
            "home.couple_story",
            "L'année était 2017. Nous venions tous les deux d'arriver à Paris ; Mouna finissait ses études et Mauro commençait son doctorat. Tout a commencé par un dîner, puis une amitié, une complicité. Ce qui devait être une simple rencontre a marqué le début de notre histoire.",
        );
        map.insert("home.see_you_there", "À bientôt !");

        // Events page
        map.insert(
            "events.subtitle_single",
            "Rejoignez-nous pour célébrer notre amour",
        );
        map.insert(
            "events.subtitle_multiple",
            "Rejoignez-nous pour célébrer notre amour à travers de belles destinations",
        );
        map.insert("events.title", "Détails de l'événement");
        map.insert("events.sardinia", "Sardaigne, Italie");
        map.insert("events.tunisia", "Tunisie");
        map.insert("events.nice", "Nice, France");
        map.insert("events.schedule", "Programme");
        map.insert("events.venue", "Lieu");
        map.insert("events.accommodation", "Hébergement");
        map.insert("events.travel", "Informations de voyage");
        map.insert("events.view_on_maps", "Voir sur Google Maps");

        // Event content placeholders
        map.insert("events.date_sardinia", "19 Septembre 2026");
        map.insert("events.sort_date_sardinia", "2026-09-19");
        map.insert("events.schedule_sardinia", "Cérémonie à 18h00");
        map.insert("events.venue_sardinia_name", "Sa Mola Hotel Ristorante");
        map.insert(
            "events.venue_sardinia_link",
            "https://maps.app.goo.gl/yNLukc3C9V6bPL4DA",
        );
        map.insert(
            "events.accommodation_sardinia",
            "• Hôtels recommandés à <a href='https://www.google.com/maps/place/Oristano,+OR,+Italy' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Oristano</a> / <a href='https://www.google.com/maps/place/Cabras,+OR,+Italy' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Cabras</a>",
        );
        map.insert("events.travel_sardinia", "• L'aéroport de Cagliari (CA) est le plus proche<br/>• Nous recommandons de louer une voiture");
        map.insert("events.date_tunisia", "27 Juin 2026");
        map.insert("events.sort_date_tunisia", "2026-06-27");
        map.insert("events.schedule_tunisia", "Début à 21h00");
        map.insert("events.venue_tunisia_name", "Espace La Vallée, Monastir");
        map.insert(
            "events.venue_tunisia_link",
            "https://maps.app.goo.gl/Y4dCfdekMGiWvMFX6",
        );
        map.insert("events.accommodation_tunisia", "• Hôtels à Monastir<br/>• Les hôtels peuvent également être réservés dans la <a href='https://www.google.com/maps/d/edit?mid=1saWGZmjkgOkyQZxfyFeMldJy3JWWvg8&usp=sharing' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>zone touristique de Monastir</a>");
        map.insert(
            "events.travel_tunisia",
            "• Aéroport le plus proche : Monastir<br/>• Nous recommandons d'éviter la compagnie aérienne Tunisair<br/>• L'eau du robinet n'est pas potable en Tunisie",
        );
        map.insert("events.date_nice", "8 Avril 2026");
        map.insert("events.sort_date_nice", "2026-04-08");
        map.insert(
            "events.schedule_nice",
            "Cérémonie à 11h00, suivi d'un déjeuner",
        );
        map.insert("events.venue_nice_name", "Mairie de Nice");
        map.insert(
            "events.venue_nice_link",
            "https://maps.app.goo.gl/D9hQbstQqHWxa1m49",
        );
        map.insert("events.accommodation_nice", "• Hôtels à Nice<br/>• Privilégiez un hébergement le long des lignes de tram 2 et 3, de préférence à proximité de la plage");
        map.insert(
            "events.travel_nice",
            "• L'<a href='https://maps.app.goo.gl/8KRRidQakgL2C97t8' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>aéroport Nice Côte d'Azur</a> dessert la région",
        );

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
        map.insert("rsvp.success", "Merci! Votre RSVP a été enregistré.");
        map.insert("rsvp.success_thank_you", "Merci pour votre réponse !");
        map.insert(
            "rsvp.success_refresh",
            "La page se rechargera dans 5 secondes.",
        );
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
        map.insert("rsvp.guest_list", "Liste des invités");
        map.insert(
            "rsvp.guest_list_help",
            "Ajoutez tous les invités de votre groupe et leurs préférences alimentaires",
        );
        map.insert("rsvp.add_another_guest", "+ Ajouter un autre invité");
        map.insert(
            "rsvp.notes_help",
            "Des demandes spéciales, restrictions alimentaires ou messages pour nous ?",
        );
        map.insert(
            "rsvp.notes_placeholder",
            "Des demandes spéciales ou messages ?",
        );
        map.insert("rsvp.attending_label", "Présent(e) à :");
        map.insert("rsvp.age_category", "Catégorie d'âge :");
        map.insert("rsvp.adult", "Adulte");
        map.insert("rsvp.child_under_3", "< 3 ans");
        map.insert("rsvp.child_under_10", "< 10 ans");
        map.insert(
            "rsvp.dietary_restrictions_label",
            "Restrictions alimentaires :",
        );
        map.insert("rsvp.guest_not_found", "Invité non trouvé");

        // Admin
        map.insert("admin.title", "Tableau de bord Admin");
        map.insert("admin.login", "Connexion");
        map.insert("admin.logout", "Déconnexion");
        map.insert("admin.dashboard", "Tableau de bord");
        map.insert("admin.guests", "Invités");
        map.insert("admin.rsvps", "RSVPs");

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
        map.insert("events.travel_sardinia", "• L'aeroporto di Cagliari (CA) è il più vicino<br/>• Si consiglia di noleggiare un'auto");
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
            "• Aeroporto più vicino: Monastir<br/>• Raccomandiamo di evitare la compagnia aerea Tunisair<br/>• L'acqua di rubinetto non è potabile in Tunisia",
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
}
