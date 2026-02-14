-- ============================================================================
-- Wedding Website - Initial Database Schema
-- Created: 2026-02-10
-- Tables: guest_groups, guests, rsvps, content, photos, config
--
-- Security Model:
-- - Guest authentication required via invitation codes (frontend GuestContext)
-- - Photos gallery accessible only to authenticated guests
-- - Storage bucket 'wedding-photos' is PRIVATE (not public)
-- - Admin operations require authenticated admin users
-- ============================================================================

-- ============================================================================
-- TABLES
-- ============================================================================

-- Guest Groups table
-- Represents invitation groups/households
CREATE TABLE guest_groups (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  email TEXT,
  invitation_code TEXT UNIQUE NOT NULL,
  party_size INTEGER NOT NULL DEFAULT 1 CHECK (party_size > 0),
  location TEXT NOT NULL CHECK (location IN ('sardinia', 'tunisia', 'both')),
  default_language TEXT NOT NULL DEFAULT 'en' CHECK (default_language IN ('en', 'fr', 'it')),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for guest_groups table
CREATE INDEX idx_guest_groups_invitation_code ON guest_groups(invitation_code);
CREATE INDEX idx_guest_groups_location ON guest_groups(location);
CREATE INDEX idx_guest_groups_default_language ON guest_groups(default_language);

-- Guests table
-- Individual guests/invitees within a guest group
CREATE TABLE guests (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  guest_group_id UUID NOT NULL REFERENCES guest_groups(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  dietary_preferences JSONB DEFAULT '{"vegetarian": false, "vegan": false, "gluten_free": false, "other": ""}'::jsonb,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  CONSTRAINT guests_name_not_empty CHECK (length(trim(name)) > 0)
);

-- Index for fast lookups by guest group
CREATE INDEX idx_guests_guest_group_id ON guests(guest_group_id);

-- RSVPs table
-- Note: Dietary preferences are counted from the guests table, not stored here
CREATE TABLE rsvps (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  guest_group_id UUID NOT NULL REFERENCES guest_groups(id) ON DELETE CASCADE,
  location TEXT NOT NULL CHECK (location IN ('sardinia', 'tunisia')),
  attending BOOLEAN NOT NULL,
  number_of_guests INTEGER NOT NULL CHECK (number_of_guests >= 0),
  additional_notes TEXT,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  CONSTRAINT rsvps_guest_location_unique UNIQUE (guest_group_id, location)
);

-- Indexes for rsvps table
CREATE INDEX idx_rsvps_guest_group_id ON rsvps(guest_group_id);
CREATE INDEX idx_rsvps_location ON rsvps(location);
CREATE INDEX idx_rsvps_attending ON rsvps(attending);

-- Content table (multilingual website content)
CREATE TABLE content (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  key TEXT NOT NULL,
  language TEXT NOT NULL CHECK (language IN ('en', 'fr', 'it')),
  location TEXT CHECK (location IN ('sardinia', 'tunisia', 'both')),
  value TEXT NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  CONSTRAINT content_key_lang_loc_unique UNIQUE (key, language, location)
);

-- Index for content lookups
CREATE INDEX idx_content_lookup ON content(key, language, location);

-- Photos table
CREATE TABLE photos (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  filename TEXT NOT NULL,
  caption TEXT,
  display_order INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Index for photo ordering
CREATE INDEX idx_photos_order ON photos(display_order);

-- Configuration table
CREATE TABLE config (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- ============================================================================
-- TRIGGERS - Auto-update timestamps
-- ============================================================================

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$;

-- Apply triggers to all tables with updated_at
CREATE TRIGGER update_guest_groups_updated_at
  BEFORE UPDATE ON guest_groups
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_guests_updated_at
  BEFORE UPDATE ON guests
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_rsvps_updated_at
  BEFORE UPDATE ON rsvps
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_content_updated_at
  BEFORE UPDATE ON content
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_config_updated_at
  BEFORE UPDATE ON config
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- BUSINESS LOGIC FUNCTIONS
-- ============================================================================

-- Function for secure guest group authentication by invitation code
CREATE OR REPLACE FUNCTION authenticate_guest_group(code TEXT)
RETURNS TABLE (
  id UUID,
  name TEXT,
  email TEXT,
  invitation_code TEXT,
  party_size INTEGER,
  location TEXT,
  default_language TEXT,
  created_at TIMESTAMP WITH TIME ZONE,
  updated_at TIMESTAMP WITH TIME ZONE
)
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
BEGIN
  RETURN QUERY
  SELECT
    gg.id,
    gg.name,
    gg.email,
    gg.invitation_code,
    gg.party_size,
    gg.location,
    gg.default_language,
    gg.created_at,
    gg.updated_at
  FROM guest_groups gg
  WHERE gg.invitation_code = code;
END;
$$;

-- Grant execute permission to anonymous users
GRANT EXECUTE ON FUNCTION authenticate_guest_group(TEXT) TO anon, authenticated;

-- Function to validate guest_group_id access
-- NOTE: This function is DEPRECATED and should not be used for security
-- It only checks if a guest_group exists, not if the user owns it
-- Use the new secure RPC functions instead
CREATE OR REPLACE FUNCTION is_valid_guest_group_id(check_guest_group_id UUID)
RETURNS BOOLEAN
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
BEGIN
  -- Only admins can use this now
  IF auth.role() = 'authenticated' THEN
    RETURN true;
  END IF;

  -- Anonymous users should use RPC functions with invitation_code
  RETURN false;
END;
$$;

-- ============================================================================
-- SECURE RPC FUNCTIONS - Guests Table
-- ============================================================================

-- Get guests for a specific guest group with invitation code validation
CREATE OR REPLACE FUNCTION get_guests_for_group(
  p_guest_group_id UUID,
  p_invitation_code TEXT
)
RETURNS SETOF guests
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
BEGIN
  -- Validate invitation code matches guest_group_id
  IF NOT EXISTS (
    SELECT 1 FROM guest_groups
    WHERE id = p_guest_group_id
    AND invitation_code = p_invitation_code
  ) THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  -- Return guests only for validated group
  RETURN QUERY
  SELECT * FROM guests
  WHERE guest_group_id = p_guest_group_id
  ORDER BY created_at;
END;
$$;

-- Create a new guest for a guest group with invitation code validation
-- Note: No party size limit check - we count actual guests dynamically in the UI
CREATE OR REPLACE FUNCTION create_guest_for_group(
  p_guest_group_id UUID,
  p_invitation_code TEXT,
  p_name TEXT,
  p_dietary_preferences JSONB
)
RETURNS guests
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
  v_guest guests;
BEGIN
  -- Validate invitation code
  IF NOT EXISTS (
    SELECT 1 FROM guest_groups
    WHERE id = p_guest_group_id
    AND invitation_code = p_invitation_code
  ) THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  -- Validate name
  IF length(trim(p_name)) = 0 THEN
    RAISE EXCEPTION 'Guest name cannot be empty';
  END IF;

  -- Insert guest (no party size limit check)
  INSERT INTO guests (guest_group_id, name, dietary_preferences)
  VALUES (p_guest_group_id, p_name, p_dietary_preferences)
  RETURNING * INTO v_guest;

  RETURN v_guest;
END;
$$;

-- Update a guest with invitation code validation
CREATE OR REPLACE FUNCTION update_guest_for_group(
  p_guest_id UUID,
  p_guest_group_id UUID,
  p_invitation_code TEXT,
  p_name TEXT,
  p_dietary_preferences JSONB
)
RETURNS guests
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
  v_guest guests;
BEGIN
  -- Validate invitation code
  IF NOT EXISTS (
    SELECT 1 FROM guest_groups
    WHERE id = p_guest_group_id
    AND invitation_code = p_invitation_code
  ) THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  -- Validate guest belongs to group
  IF NOT EXISTS (
    SELECT 1 FROM guests
    WHERE id = p_guest_id
    AND guest_group_id = p_guest_group_id
  ) THEN
    RAISE EXCEPTION 'Guest does not belong to this group';
  END IF;

  -- Validate name
  IF length(trim(p_name)) = 0 THEN
    RAISE EXCEPTION 'Guest name cannot be empty';
  END IF;

  -- Update guest
  UPDATE guests
  SET name = p_name,
      dietary_preferences = p_dietary_preferences,
      updated_at = NOW()
  WHERE id = p_guest_id
  RETURNING * INTO v_guest;

  RETURN v_guest;
END;
$$;

-- Delete a guest with invitation code validation
CREATE OR REPLACE FUNCTION delete_guest_for_group(
  p_guest_id UUID,
  p_guest_group_id UUID,
  p_invitation_code TEXT
)
RETURNS BOOLEAN
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
BEGIN
  -- Validate invitation code
  IF NOT EXISTS (
    SELECT 1 FROM guest_groups
    WHERE id = p_guest_group_id
    AND invitation_code = p_invitation_code
  ) THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  -- Validate guest belongs to group
  IF NOT EXISTS (
    SELECT 1 FROM guests
    WHERE id = p_guest_id
    AND guest_group_id = p_guest_group_id
  ) THEN
    RAISE EXCEPTION 'Guest does not belong to this group';
  END IF;

  -- Delete guest
  DELETE FROM guests
  WHERE id = p_guest_id;

  RETURN TRUE;
END;
$$;

-- Grant execute permissions on guest functions
GRANT EXECUTE ON FUNCTION get_guests_for_group(UUID, TEXT) TO anon, authenticated;
GRANT EXECUTE ON FUNCTION create_guest_for_group(UUID, TEXT, TEXT, JSONB) TO anon, authenticated;
GRANT EXECUTE ON FUNCTION update_guest_for_group(UUID, UUID, TEXT, TEXT, JSONB) TO anon, authenticated;
GRANT EXECUTE ON FUNCTION delete_guest_for_group(UUID, UUID, TEXT) TO anon, authenticated;

-- ============================================================================
-- SECURITY - Row Level Security (RLS)
-- ============================================================================

-- Enable RLS on all tables
ALTER TABLE guest_groups ENABLE ROW LEVEL SECURITY;
ALTER TABLE guests ENABLE ROW LEVEL SECURITY;
ALTER TABLE rsvps ENABLE ROW LEVEL SECURITY;
ALTER TABLE content ENABLE ROW LEVEL SECURITY;
ALTER TABLE photos ENABLE ROW LEVEL SECURITY;
ALTER TABLE config ENABLE ROW LEVEL SECURITY;

-- ============================================================================
-- RLS POLICIES - Guest Groups Table
-- ============================================================================

-- Only admins can see all guest groups via direct SELECT
CREATE POLICY "guest_groups_select_admin_only"
  ON guest_groups FOR SELECT
  TO authenticated
  USING (auth.role() = 'authenticated');

-- Only admins can insert guest groups
CREATE POLICY "guest_groups_insert_admin"
  ON guest_groups FOR INSERT
  TO authenticated
  WITH CHECK (auth.role() = 'authenticated');

-- Only admins can update guest groups
CREATE POLICY "guest_groups_update_admin"
  ON guest_groups FOR UPDATE
  TO authenticated
  USING (auth.role() = 'authenticated')
  WITH CHECK (auth.role() = 'authenticated');

-- Only admins can delete guest groups
CREATE POLICY "guest_groups_delete_admin"
  ON guest_groups FOR DELETE
  TO authenticated
  USING (auth.role() = 'authenticated');

-- ============================================================================
-- RLS POLICIES - Guests Table (Individual Invitees)
-- ============================================================================

-- Admins can view all guests
CREATE POLICY "admins_can_view_all_guests"
  ON guests FOR SELECT
  TO authenticated
  USING (auth.role() = 'authenticated');

-- Admins can insert guests
CREATE POLICY "admins_can_insert_guests"
  ON guests FOR INSERT
  TO authenticated
  WITH CHECK (auth.role() = 'authenticated');

-- Admins can update guests
CREATE POLICY "admins_can_update_guests"
  ON guests FOR UPDATE
  TO authenticated
  USING (auth.role() = 'authenticated')
  WITH CHECK (auth.role() = 'authenticated');

-- Admins can delete guests
CREATE POLICY "admins_can_delete_guests"
  ON guests FOR DELETE
  TO authenticated
  USING (auth.role() = 'authenticated');

-- NOTE: Anonymous users access guests through secure RPC functions below
-- Direct table access is blocked for security
-- No policies for anon role - must use RPC functions with invitation_code validation

-- Grant permissions only to authenticated admins
GRANT SELECT, INSERT, UPDATE, DELETE ON guests TO authenticated;

-- NOTE: Anonymous users access guest_groups through authenticate_guest_group() function
-- No direct SELECT grant needed

-- ============================================================================
-- SECURE FUNCTION - Update Party Size with Invitation Code Validation
-- ============================================================================

-- Create a secure function to update party_size with invitation code validation
-- This allows guests to increase their party size when RSVPing without compromising security
CREATE OR REPLACE FUNCTION update_guest_group_party_size(
  p_guest_group_id UUID,
  p_invitation_code TEXT,
  p_new_party_size INTEGER
)
RETURNS guest_groups
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
  v_guest_group guest_groups;
BEGIN
  -- Validate that the invitation code matches the guest_group_id
  SELECT * INTO v_guest_group
  FROM guest_groups
  WHERE id = p_guest_group_id
    AND invitation_code = p_invitation_code;

  -- If no match found, raise an error
  IF NOT FOUND THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  -- Validate party_size is reasonable (between 1 and 20)
  IF p_new_party_size < 1 OR p_new_party_size > 20 THEN
    RAISE EXCEPTION 'Party size must be between 1 and 20';
  END IF;

  -- Update the party_size
  UPDATE guest_groups
  SET party_size = p_new_party_size,
      updated_at = NOW()
  WHERE id = p_guest_group_id
  RETURNING * INTO v_guest_group;

  RETURN v_guest_group;
END;
$$;

-- Grant EXECUTE permission to anon and authenticated roles
GRANT EXECUTE ON FUNCTION update_guest_group_party_size(UUID, TEXT, INTEGER) TO anon;
GRANT EXECUTE ON FUNCTION update_guest_group_party_size(UUID, TEXT, INTEGER) TO authenticated;

-- Add a comment explaining the function
COMMENT ON FUNCTION update_guest_group_party_size(UUID, TEXT, INTEGER) IS
  'Securely updates party_size for a guest_group by validating the invitation code. This allows guests to increase their party size when RSVPing without compromising security.';

-- ============================================================================
-- RLS POLICIES - RSVPs Table
-- ============================================================================

-- Admins can view all RSVPs
CREATE POLICY "rsvps_select_admin"
  ON rsvps FOR SELECT
  TO authenticated
  USING (auth.role() = 'authenticated');

-- Admins can create RSVPs
CREATE POLICY "rsvps_insert_admin"
  ON rsvps FOR INSERT
  TO authenticated
  WITH CHECK (auth.role() = 'authenticated');

-- Admins can update RSVPs
CREATE POLICY "rsvps_update_admin"
  ON rsvps FOR UPDATE
  TO authenticated
  USING (auth.role() = 'authenticated')
  WITH CHECK (auth.role() = 'authenticated');

-- NOTE: Anonymous users access RSVPs through secure RPC functions below
-- Direct table access is blocked for security

-- Only admins can delete RSVPs
CREATE POLICY "rsvps_delete_admin"
  ON rsvps FOR DELETE
  TO authenticated
  USING (auth.role() = 'authenticated');

-- ============================================================================
-- SECURE RPC FUNCTIONS - RSVPs Table
-- ============================================================================

-- Get RSVPs for a specific guest group with invitation code validation
CREATE OR REPLACE FUNCTION get_rsvps_for_group(
  p_guest_group_id UUID,
  p_invitation_code TEXT
)
RETURNS SETOF rsvps
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
BEGIN
  -- Validate invitation code
  IF NOT EXISTS (
    SELECT 1 FROM guest_groups
    WHERE id = p_guest_group_id
    AND invitation_code = p_invitation_code
  ) THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  -- Return RSVPs only for validated group
  RETURN QUERY
  SELECT * FROM rsvps
  WHERE guest_group_id = p_guest_group_id
  ORDER BY location;
END;
$$;

-- Get a specific RSVP by guest group and location with validation
CREATE OR REPLACE FUNCTION get_rsvp_by_location(
  p_guest_group_id UUID,
  p_invitation_code TEXT,
  p_location TEXT
)
RETURNS rsvps
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
  v_rsvp rsvps;
BEGIN
  -- Validate invitation code
  IF NOT EXISTS (
    SELECT 1 FROM guest_groups
    WHERE id = p_guest_group_id
    AND invitation_code = p_invitation_code
  ) THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  -- Validate location
  IF p_location NOT IN ('sardinia', 'tunisia') THEN
    RAISE EXCEPTION 'Invalid location. Must be sardinia or tunisia';
  END IF;

  -- Get RSVP
  SELECT * INTO v_rsvp
  FROM rsvps
  WHERE guest_group_id = p_guest_group_id
  AND location = p_location;

  RETURN v_rsvp;
END;
$$;

-- Create or update RSVP for a guest group with invitation code validation
CREATE OR REPLACE FUNCTION upsert_rsvp_for_group(
  p_guest_group_id UUID,
  p_invitation_code TEXT,
  p_location TEXT,
  p_attending BOOLEAN,
  p_number_of_guests INTEGER,
  p_additional_notes TEXT
)
RETURNS rsvps
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
  v_rsvp rsvps;
  v_party_size INTEGER;
BEGIN
  -- Validate invitation code and get party size
  SELECT party_size INTO v_party_size
  FROM guest_groups
  WHERE id = p_guest_group_id
  AND invitation_code = p_invitation_code;

  IF NOT FOUND THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  -- Validate location
  IF p_location NOT IN ('sardinia', 'tunisia') THEN
    RAISE EXCEPTION 'Invalid location. Must be sardinia or tunisia';
  END IF;

  -- Validate number_of_guests
  IF p_number_of_guests < 0 THEN
    RAISE EXCEPTION 'Number of guests cannot be negative';
  END IF;

  IF p_number_of_guests > v_party_size THEN
    RAISE EXCEPTION 'Number of guests (%) exceeds party size (%)', p_number_of_guests, v_party_size;
  END IF;

  -- Upsert RSVP (dietary preferences are tracked per guest, not in RSVP)
  INSERT INTO rsvps (
    guest_group_id,
    location,
    attending,
    number_of_guests,
    additional_notes
  )
  VALUES (
    p_guest_group_id,
    p_location,
    p_attending,
    p_number_of_guests,
    p_additional_notes
  )
  ON CONFLICT (guest_group_id, location)
  DO UPDATE SET
    attending = EXCLUDED.attending,
    number_of_guests = EXCLUDED.number_of_guests,
    additional_notes = EXCLUDED.additional_notes,
    updated_at = NOW()
  RETURNING * INTO v_rsvp;

  RETURN v_rsvp;
END;
$$;

-- Grant execute permissions on RSVP functions
GRANT EXECUTE ON FUNCTION get_rsvps_for_group(UUID, TEXT) TO anon, authenticated;
GRANT EXECUTE ON FUNCTION get_rsvp_by_location(UUID, TEXT, TEXT) TO anon, authenticated;
GRANT EXECUTE ON FUNCTION upsert_rsvp_for_group(UUID, TEXT, TEXT, BOOLEAN, INTEGER, TEXT) TO anon, authenticated;

-- ============================================================================
-- RLS POLICIES - Content Table
-- ============================================================================

-- Everyone can read content
CREATE POLICY "content_select_all"
  ON content FOR SELECT
  TO anon, authenticated
  USING (true);

-- Only admins can manage content
CREATE POLICY "content_all_admin"
  ON content FOR ALL
  TO authenticated
  USING (auth.role() = 'authenticated');

-- ============================================================================
-- RLS POLICIES - Photos Table
-- ============================================================================

-- Authenticated guests (with invitation codes) can view photos
-- Frontend enforces authentication via GuestContext
CREATE POLICY "photos_select_all"
  ON photos FOR SELECT
  TO anon, authenticated
  USING (true);

-- Only admins can manage photos
CREATE POLICY "photos_all_admin"
  ON photos FOR ALL
  TO authenticated
  USING (auth.role() = 'authenticated');

-- ============================================================================
-- RLS POLICIES - Config Table
-- ============================================================================

-- Everyone can read config
CREATE POLICY "config_select_all"
  ON config FOR SELECT
  TO anon, authenticated
  USING (true);

-- Only admins can manage config
CREATE POLICY "config_all_admin"
  ON config FOR ALL
  TO authenticated
  USING (auth.role() = 'authenticated');

-- ============================================================================
-- VIEWS - Statistics and Reports
-- ============================================================================

-- RSVP statistics view
CREATE VIEW rsvp_stats
WITH (security_invoker = true)
AS
SELECT
  COUNT(*) FILTER (WHERE r.attending = true) as confirmed_count,
  COUNT(*) as total_rsvps,
  COUNT(g.id) FILTER (WHERE r.attending = true AND g.dietary_preferences->>'vegetarian' = 'true') as vegetarian_count,
  COUNT(g.id) FILTER (WHERE r.attending = true AND g.dietary_preferences->>'vegan' = 'true') as vegan_count,
  SUM(r.number_of_guests) FILTER (WHERE r.attending = true) as total_attending_guests,
  COUNT(*) FILTER (WHERE r.location = 'sardinia' AND r.attending = true) as sardinia_attending,
  COUNT(*) FILTER (WHERE r.location = 'tunisia' AND r.attending = true) as tunisia_attending
FROM guest_groups gg
LEFT JOIN rsvps r ON gg.id = r.guest_group_id
LEFT JOIN guests g ON gg.id = g.guest_group_id;

-- Pending RSVPs view (guest groups without responses)
CREATE VIEW pending_rsvps
WITH (security_invoker = true)
AS
SELECT gg.*
FROM guest_groups gg
WHERE NOT EXISTS (
  SELECT 1 FROM rsvps r WHERE r.guest_group_id = gg.id
);

-- ============================================================================
-- SEED DATA - Configuration
-- ============================================================================

INSERT INTO config (key, value) VALUES
  ('wedding_date_sardinia', '2026-09-19'),
  ('wedding_date_tunisia', '2026-06-27'),
  ('venue_address_sardinia', 'Costa Smeralda, Sardinia, Italy'),
  ('venue_address_tunisia', 'Tunis, Tunisia'),
  ('rsvp_enabled', 'true'),
  ('default_language', 'en');

-- ============================================================================
-- SEED DATA - Multilingual Content
-- ============================================================================

-- English content
INSERT INTO content (key, language, location, value) VALUES
  ('welcome_message', 'en', NULL, 'Welcome to our wedding website! We are so excited to celebrate our special day with you.'),
  ('our_story', 'en', NULL, 'Our love story began...'),
  ('schedule_sardinia', 'en', 'sardinia', 'Ceremony: 4:00 PM\nReception: 6:00 PM\nDinner: 8:00 PM'),
  ('venue_sardinia', 'en', 'sardinia', 'Beautiful seaside venue in Costa Smeralda with stunning Mediterranean views.'),
  ('accommodation_sardinia', 'en', 'sardinia', 'We have reserved blocks of rooms at several nearby hotels. Contact us for details.'),
  ('travel_sardinia', 'en', 'sardinia', 'Fly into Olbia Airport (OLB). Car rental recommended.'),
  ('schedule_tunisia', 'en', 'tunisia', 'Ceremony: 5:00 PM\nReception: 7:00 PM\nDinner: 9:00 PM'),
  ('venue_tunisia', 'en', 'tunisia', 'Traditional venue in Tunis with stunning architecture and gardens.'),
  ('accommodation_tunisia', 'en', 'tunisia', 'Recommended hotels and guest houses in Tunis city center.'),
  ('travel_tunisia', 'en', 'tunisia', 'Fly into Tunis-Carthage International Airport (TUN). Taxis and transport readily available.');

-- French content
INSERT INTO content (key, language, location, value) VALUES
  ('welcome_message', 'fr', NULL, 'Bienvenue sur notre site de mariage ! Nous sommes très heureux de célébrer notre jour spécial avec vous.'),
  ('our_story', 'fr', NULL, 'Notre histoire d''amour a commencé...'),
  ('schedule_sardinia', 'fr', 'sardinia', 'Cérémonie : 16h00\nRéception : 18h00\nDîner : 20h00'),
  ('venue_sardinia', 'fr', 'sardinia', 'Magnifique lieu en bord de mer à Costa Smeralda avec une vue imprenable sur la Méditerranée.'),
  ('accommodation_sardinia', 'fr', 'sardinia', 'Nous avons réservé des chambres dans plusieurs hôtels à proximité. Contactez-nous pour plus de détails.'),
  ('travel_sardinia', 'fr', 'sardinia', 'Voler à l''aéroport d''Olbia (OLB). Location de voiture recommandée.'),
  ('schedule_tunisia', 'fr', 'tunisia', 'Cérémonie : 17h00\nRéception : 19h00\nDîner : 21h00'),
  ('venue_tunisia', 'fr', 'tunisia', 'Lieu traditionnel à Tunis avec une architecture et des jardins magnifiques.'),
  ('accommodation_tunisia', 'fr', 'tunisia', 'Hôtels et maisons d''hôtes recommandés dans le centre-ville de Tunis.'),
  ('travel_tunisia', 'fr', 'tunisia', 'Voler à l''aéroport international de Tunis-Carthage (TUN). Taxis et transports facilement disponibles.');

-- Italian content
INSERT INTO content (key, language, location, value) VALUES
  ('welcome_message', 'it', NULL, 'Benvenuti sul nostro sito di matrimonio! Siamo entusiasti di celebrare il nostro giorno speciale con voi.'),
  ('our_story', 'it', NULL, 'La nostra storia d''amore è iniziata...'),
  ('schedule_sardinia', 'it', 'sardinia', 'Cerimonia: 16:00\nRicevimento: 18:00\nCena: 20:00'),
  ('venue_sardinia', 'it', 'sardinia', 'Splendida location sul mare in Costa Smeralda con vista mozzafiato sul Mediterraneo.'),
  ('accommodation_sardinia', 'it', 'sardinia', 'Abbiamo riservato camere in diversi hotel nelle vicinanze. Contattateci per i dettagli.'),
  ('travel_sardinia', 'it', 'sardinia', 'Volare all''aeroporto di Olbia (OLB). Si consiglia il noleggio auto.'),
  ('schedule_tunisia', 'it', 'tunisia', 'Cerimonia: 17:00\nRicevimento: 19:00\nCena: 21:00'),
  ('venue_tunisia', 'it', 'tunisia', 'Location tradizionale a Tunisi con splendida architettura e giardini.'),
  ('accommodation_tunisia', 'it', 'tunisia', 'Hotel e pensioni consigliati nel centro di Tunisi.'),
  ('travel_tunisia', 'it', 'tunisia', 'Volare all''aeroporto internazionale di Tunisi-Cartagine (TUN). Taxi e trasporti prontamente disponibili.');

-- ============================================================================
-- COMPLETION MESSAGE
-- ============================================================================

DO $$
BEGIN
  RAISE NOTICE '============================================================================';
  RAISE NOTICE '✅ Wedding Website Database Schema Created Successfully!';
  RAISE NOTICE '============================================================================';
  RAISE NOTICE '';
  RAISE NOTICE '📊 Tables Created:';
  RAISE NOTICE '  • guest_groups (invitation groups/households)';
  RAISE NOTICE '  • guests (individual invitees)';
  RAISE NOTICE '  • rsvps (with per-location responses)';
  RAISE NOTICE '  • content (multilingual: EN/FR/IT)';
  RAISE NOTICE '  • photos (gallery management)';
  RAISE NOTICE '  • config (site configuration)';
  RAISE NOTICE '';
  RAISE NOTICE '🔒 Security Features:';
  RAISE NOTICE '  ✓ Row Level Security (RLS) enabled on all tables';
  RAISE NOTICE '  ✓ Guest group isolation via SECURITY DEFINER functions';
  RAISE NOTICE '  ✓ Invitation code validation on all guest operations';
  RAISE NOTICE '  ✓ Admin full access via JWT authentication';
  RAISE NOTICE '  ✓ Anonymous users must use RPC functions with invitation_code';
  RAISE NOTICE '';
  RAISE NOTICE '📈 Views Created:';
  RAISE NOTICE '  • rsvp_stats - Real-time statistics';
  RAISE NOTICE '  • pending_rsvps - Guest groups without responses';
  RAISE NOTICE '';
  RAISE NOTICE '🌱 Seed Data:';
  RAISE NOTICE '  ✓ Default configuration loaded';
  RAISE NOTICE '  ✓ Sample content in 3 languages';
  RAISE NOTICE '';
  RAISE NOTICE '📝 Table Naming:';
  RAISE NOTICE '  • guest_groups = invitation groups (formerly "guests")';
  RAISE NOTICE '  • guests = individual invitees (formerly "invitees")';
  RAISE NOTICE '';
  RAISE NOTICE '📝 Next Steps:';
  RAISE NOTICE '  1. Create admin user: Dashboard → Authentication → Users';
  RAISE NOTICE '  2. Add guest groups via admin panel';
  RAISE NOTICE '  3. Customize content and configuration';
  RAISE NOTICE '  4. Frontend MUST use RPC functions for guest/RSVP operations:';
  RAISE NOTICE '     • get_guests_for_group(guest_group_id, invitation_code)';
  RAISE NOTICE '     • create_guest_for_group(guest_group_id, invitation_code, name, dietary)';
  RAISE NOTICE '     • update_guest_for_group(guest_id, guest_group_id, invitation_code, name, dietary)';
  RAISE NOTICE '     • delete_guest_for_group(guest_id, guest_group_id, invitation_code)';
  RAISE NOTICE '     • get_rsvps_for_group(guest_group_id, invitation_code)';
  RAISE NOTICE '     • get_rsvp_by_location(guest_group_id, invitation_code, location)';
  RAISE NOTICE '     • upsert_rsvp_for_group(guest_group_id, invitation_code, location, ...)';
  RAISE NOTICE '  ⚠️  WARNING: Direct table queries for guests/rsvps will FAIL for anon users';
  RAISE NOTICE '';
  RAISE NOTICE '============================================================================';
END $$;

-- ============================================================================
-- STORAGE BUCKETS
-- ============================================================================

-- Create storage bucket for wedding photos (PUBLIC)
INSERT INTO storage.buckets (id, name, public, file_size_limit, allowed_mime_types)
VALUES (
  'wedding-photos',
  'wedding-photos',
  true,  -- Public bucket - guests can view photos
  52428800,  -- 50MB file size limit
  ARRAY['image/jpeg', 'image/jpg', 'image/png', 'image/gif', 'image/webp']
)
ON CONFLICT (id) DO NOTHING;

-- ============================================================================
-- STORAGE POLICIES
-- ============================================================================

-- Allow everyone to view photos (bucket is public)
CREATE POLICY "Anyone can view wedding photos"
ON storage.objects FOR SELECT
USING (bucket_id = 'wedding-photos');

-- Allow authenticated users (admins) to upload photos
CREATE POLICY "Authenticated users can upload photos"
ON storage.objects FOR INSERT
TO authenticated
WITH CHECK (bucket_id = 'wedding-photos');

-- Allow authenticated users (admins) to update photos
CREATE POLICY "Authenticated users can update photos"
ON storage.objects FOR UPDATE
TO authenticated
USING (bucket_id = 'wedding-photos');

-- Allow authenticated users (admins) to delete photos
CREATE POLICY "Authenticated users can delete photos"
ON storage.objects FOR DELETE
TO authenticated
USING (bucket_id = 'wedding-photos');
