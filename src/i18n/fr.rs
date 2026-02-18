//! French translations.

use std::collections::HashMap;

pub fn translations() -> HashMap<&'static str, &'static str> {
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
        "• Nous recommandons de consulter les hôtels à <a href='https://maps.app.goo.gl/N4KVpYEZF7G4jWbC6' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Oristano</a> / <a href='https://maps.app.goo.gl/x72Q9zfYCDEZWMem9' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Cabras</a>",
    );
    map.insert("events.travel_sardinia", "• Aéroport le plus proche : <a href='https://maps.app.goo.gl/uvofAX2NkqLeoi2D7' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Cagliari</a><br/>• Nous recommandons de louer une voiture");
    map.insert("events.date_tunisia", "27 Juin 2026");
    map.insert("events.sort_date_tunisia", "2026-06-27");
    map.insert("events.schedule_tunisia", "Début à 21h00");
    map.insert("events.venue_tunisia_name", "Espace La Vallée, Monastir");
    map.insert(
        "events.venue_tunisia_link",
        "https://maps.app.goo.gl/Y4dCfdekMGiWvMFX6",
    );
    map.insert("events.accommodation_tunisia", "• Hôtels à Monastir<br/>• Nous recommandons de consulter la <a href='https://www.google.com/maps/d/edit?mid=1saWGZmjkgOkyQZxfyFeMldJy3JWWvg8&usp=sharing' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>zone touristique de Monastir</a>");
    map.insert(
        "events.travel_tunisia",
        "• Aéroport le plus proche : <a href='https://maps.app.goo.gl/YyvPgoUmRDPqmzgy8' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Monastir</a><br/>• Nous recommandons d'éviter la compagnie aérienne Tunisair<br/>• L'eau du robinet n'est pas potable en Tunisie",
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
        "• Aéroport le plus proche : <a href='https://maps.app.goo.gl/8KRRidQakgL2C97t8' target='_blank' class='text-secondary-700 underline hover:text-secondary-900'>Nice Côte d'Azur</a>",
    );

    // Location names (short, for RSVP checkboxes etc.)
    map.insert("location.sardinia", "Sardaigne");
    map.insert("location.tunisia", "Tunisie");
    map.insert("location.nice", "Nice");

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
        "Actualisation de la page dans 5 secondes.",
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
        "rsvp.error_empty_names",
        "Veuillez remplir les noms de tous les invités avant de soumettre.",
    );
    map.insert(
        "rsvp.error_no_locations",
        "Veuillez sélectionner au moins un lieu pour chaque invité.",
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
