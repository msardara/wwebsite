-- ============================================================================
-- Wedding Website - Initial Database Schema
-- Created: 2026-02-10
-- Tables: guest_groups, guests
--
-- Security Model:
-- - Guest authentication required via invitation codes (frontend GuestContext)
-- - Admin operations require authenticated admin users
-- ============================================================================

-- ============================================================================
-- VALIDATION FUNCTIONS
-- ============================================================================

-- Valid wedding locations constant
-- Returns the canonical array of allowed location values.
-- Used by CHECK constraints and validation logic throughout the schema.
CREATE OR REPLACE FUNCTION valid_locations()
RETURNS TEXT[]
LANGUAGE sql
IMMUTABLE
AS $$
  SELECT ARRAY['sardinia', 'tunisia', 'nice']::text[];
$$;

-- Valid age categories constant
-- Returns the canonical array of allowed age category values.
-- Used by CHECK constraints and validation logic throughout the schema.
CREATE OR REPLACE FUNCTION valid_age_categories()
RETURNS TEXT[]
LANGUAGE sql
IMMUTABLE
AS $$
  SELECT ARRAY['adult', 'child_under_3', 'child_under_10']::text[];
$$;

-- Validate dietary_preferences JSONB structure
-- Allowed keys: vegetarian, vegan, halal, no_pork, gluten_free (boolean), other (string)
CREATE OR REPLACE FUNCTION validate_dietary_preferences(prefs JSONB)
RETURNS BOOLEAN
LANGUAGE plpgsql
IMMUTABLE
AS $$
DECLARE
  allowed_keys TEXT[] := ARRAY['vegetarian', 'vegan', 'halal', 'no_pork', 'gluten_free', 'other'];
  key TEXT;
BEGIN
  -- Allow NULL
  IF prefs IS NULL THEN
    RETURN TRUE;
  END IF;

  -- Must be a JSON object
  IF jsonb_typeof(prefs) != 'object' THEN
    RETURN FALSE;
  END IF;

  -- Check no unexpected keys
  FOR key IN SELECT jsonb_object_keys(prefs)
  LOOP
    IF NOT (key = ANY(allowed_keys)) THEN
      RETURN FALSE;
    END IF;
  END LOOP;

  -- Validate boolean fields
  IF prefs ? 'vegetarian' AND jsonb_typeof(prefs -> 'vegetarian') != 'boolean' THEN RETURN FALSE; END IF;
  IF prefs ? 'vegan' AND jsonb_typeof(prefs -> 'vegan') != 'boolean' THEN RETURN FALSE; END IF;
  IF prefs ? 'halal' AND jsonb_typeof(prefs -> 'halal') != 'boolean' THEN RETURN FALSE; END IF;
  IF prefs ? 'no_pork' AND jsonb_typeof(prefs -> 'no_pork') != 'boolean' THEN RETURN FALSE; END IF;
  IF prefs ? 'gluten_free' AND jsonb_typeof(prefs -> 'gluten_free') != 'boolean' THEN RETURN FALSE; END IF;

  -- Validate 'other' is a string with max length 500
  IF prefs ? 'other' THEN
    IF jsonb_typeof(prefs -> 'other') != 'string' THEN RETURN FALSE; END IF;
    IF length(prefs ->> 'other') > 500 THEN RETURN FALSE; END IF;
  END IF;

  -- Limit total serialized size
  IF length(prefs::text) > 1000 THEN RETURN FALSE; END IF;

  RETURN TRUE;
END;
$$;

-- Check that a TEXT array contains no duplicate values.
-- Used by CHECK constraints to enforce array uniqueness.
CREATE OR REPLACE FUNCTION array_has_no_duplicates(arr TEXT[])
RETURNS BOOLEAN
LANGUAGE sql
IMMUTABLE
AS $$
  SELECT cardinality(arr) = cardinality(ARRAY(SELECT DISTINCT unnest(arr)));
$$;

-- ============================================================================
-- TABLES
-- ============================================================================

-- Guest Groups table
-- Represents invitation groups/households
CREATE TABLE guest_groups (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  email TEXT,
  invitation_code UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
  party_size INTEGER NOT NULL DEFAULT 1 CHECK (party_size > 0 AND party_size <= 20),
  locations TEXT[] NOT NULL,
  default_language TEXT NOT NULL DEFAULT 'en' CHECK (default_language IN ('en', 'fr', 'it')),
  additional_notes TEXT,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  CONSTRAINT guest_groups_name_not_empty CHECK (length(trim(name)) > 0),
  CONSTRAINT guest_groups_name_length CHECK (length(name) <= 200),
  CONSTRAINT guest_groups_email_length CHECK (email IS NULL OR length(email) <= 254),

  CONSTRAINT guest_groups_additional_notes_length CHECK (additional_notes IS NULL OR length(additional_notes) <= 2000),
  CONSTRAINT guest_groups_locations_valid CHECK (locations <@ valid_locations()),
  CONSTRAINT guest_groups_locations_not_empty CHECK (cardinality(locations) > 0),
  CONSTRAINT guest_groups_locations_no_duplicates CHECK (array_has_no_duplicates(locations))
);

-- Indexes for guest_groups table
-- NOTE: No explicit index on invitation_code — the UNIQUE constraint already creates one.
CREATE INDEX idx_guest_groups_locations ON guest_groups USING GIN(locations);
CREATE INDEX idx_guest_groups_default_language ON guest_groups(default_language);

-- Guests table
-- Individual guests/invitees within a guest group
CREATE TABLE guests (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  guest_group_id UUID NOT NULL REFERENCES guest_groups(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  attending_locations TEXT[] NOT NULL DEFAULT '{}',
  dietary_preferences JSONB DEFAULT '{"vegetarian": false, "vegan": false, "halal": false, "no_pork": false, "gluten_free": false, "other": ""}'::jsonb,
  age_category TEXT DEFAULT 'adult' CHECK (age_category = ANY(valid_age_categories())),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  CONSTRAINT guests_name_not_empty CHECK (length(trim(name)) > 0),
  CONSTRAINT guests_name_length CHECK (length(name) <= 200),
  CONSTRAINT guests_dietary_valid CHECK (validate_dietary_preferences(dietary_preferences)),
  CONSTRAINT guests_attending_locations_valid CHECK (attending_locations <@ valid_locations()),
  CONSTRAINT guests_attending_locations_no_duplicates CHECK (array_has_no_duplicates(attending_locations))
);

-- Indexes for guests table
CREATE INDEX idx_guests_guest_group_id ON guests(guest_group_id);
CREATE INDEX idx_guests_attending_locations ON guests USING GIN(attending_locations);
CREATE INDEX idx_guests_age_category ON guests(age_category);





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

-- Revoke default PUBLIC execute on trigger functions (defense-in-depth)
REVOKE EXECUTE ON FUNCTION update_updated_at_column() FROM PUBLIC;


-- ============================================================================
-- IMMUTABILITY TRIGGER - Prevent modification of invitation_code after creation
-- ============================================================================

CREATE OR REPLACE FUNCTION prevent_invitation_code_update()
RETURNS TRIGGER
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
BEGIN
  IF OLD.invitation_code IS DISTINCT FROM NEW.invitation_code THEN
    RAISE EXCEPTION 'invitation_code cannot be modified after creation';
  END IF;
  RETURN NEW;
END;
$$;

CREATE TRIGGER enforce_invitation_code_immutable
  BEFORE UPDATE ON guest_groups
  FOR EACH ROW
  EXECUTE FUNCTION prevent_invitation_code_update();

REVOKE EXECUTE ON FUNCTION prevent_invitation_code_update() FROM PUBLIC;

-- ============================================================================
-- BUSINESS LOGIC FUNCTIONS
-- ============================================================================

-- Function for secure guest group authentication by invitation code
CREATE OR REPLACE FUNCTION authenticate_guest_group(code UUID)
RETURNS TABLE (
  id UUID,
  name TEXT,
  email TEXT,
  party_size INTEGER,
  locations TEXT[],
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
    gg.party_size,
    gg.locations,
    gg.default_language,
    gg.created_at,
    gg.updated_at
  FROM guest_groups gg
  WHERE gg.invitation_code = code;
END;
$$;

-- Revoke default PUBLIC execute, then grant only to intended roles
REVOKE EXECUTE ON FUNCTION authenticate_guest_group(UUID) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION authenticate_guest_group(UUID) TO anon, authenticated;

-- ============================================================================
-- SECURE RPC FUNCTIONS - Guests Table
-- ============================================================================

-- Get guests for a specific guest group with invitation code validation
CREATE OR REPLACE FUNCTION get_guests_for_group(
  p_guest_group_id UUID,
  p_invitation_code UUID
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
-- Security: Maximum 20 guests per group to prevent abuse
CREATE OR REPLACE FUNCTION create_guest_for_group(
  p_guest_group_id UUID,
  p_invitation_code UUID,
  p_name TEXT,
  p_attending_locations TEXT[],
  p_dietary_preferences JSONB,
  p_age_category TEXT DEFAULT 'adult'
)
RETURNS guests
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
  v_guest guests;
  v_current_guest_count INTEGER;
  v_group_locations TEXT[];
BEGIN
  -- Validate invitation code and fetch group locations
  SELECT locations INTO v_group_locations
  FROM guest_groups
  WHERE id = p_guest_group_id
  AND invitation_code = p_invitation_code;

  IF v_group_locations IS NULL THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  -- Check current guest count (security limit: 20 guests max)
  SELECT COUNT(*) INTO v_current_guest_count
  FROM guests
  WHERE guest_group_id = p_guest_group_id;

  IF v_current_guest_count >= 20 THEN
    RAISE EXCEPTION 'Maximum of 20 guests per group allowed';
  END IF;

  -- Reject NULL parameters (PostgreSQL three-valued logic would bypass subsequent checks)
  IF p_name IS NULL THEN
    RAISE EXCEPTION 'Guest name cannot be null';
  END IF;

  -- Normalize name by trimming whitespace
  p_name := trim(p_name);

  IF p_attending_locations IS NULL THEN
    RAISE EXCEPTION 'Attending locations cannot be null';
  END IF;

  -- Deduplicate attending_locations to prevent duplicate entries
  p_attending_locations := ARRAY(SELECT DISTINCT unnest(p_attending_locations));

  IF p_age_category IS NULL THEN
    RAISE EXCEPTION 'Age category cannot be null';
  END IF;

  -- Validate name
  IF length(trim(p_name)) = 0 THEN
    RAISE EXCEPTION 'Guest name cannot be empty';
  END IF;

  IF length(p_name) > 200 THEN
    RAISE EXCEPTION 'Guest name must be 200 characters or less';
  END IF;

  -- Validate dietary_preferences
  IF NOT validate_dietary_preferences(p_dietary_preferences) THEN
    RAISE EXCEPTION 'Invalid dietary preferences: must be a JSON object with keys (vegetarian, vegan, halal, no_pork, gluten_free as booleans; other as string up to 500 chars)';
  END IF;

  -- Validate age_category
  IF NOT (p_age_category = ANY(valid_age_categories())) THEN
    RAISE EXCEPTION 'Invalid age category. Must be one of: %', array_to_string(valid_age_categories(), ', ');
  END IF;

  -- Validate attending_locations against global valid locations
  IF NOT (p_attending_locations <@ valid_locations()) THEN
    RAISE EXCEPTION 'Invalid attending locations. Each location must be one of: %', array_to_string(valid_locations(), ', ');
  END IF;

  -- Validate attending_locations against group's invited locations
  IF NOT (p_attending_locations <@ v_group_locations) THEN
    RAISE EXCEPTION 'Attending locations must be within the group''s invited locations: %', array_to_string(v_group_locations, ', ');
  END IF;

  -- Insert guest
  INSERT INTO guests (guest_group_id, name, attending_locations, dietary_preferences, age_category)
  VALUES (p_guest_group_id, p_name, p_attending_locations, p_dietary_preferences, p_age_category)
  RETURNING * INTO v_guest;

  RETURN v_guest;
END;
$$;

-- Update guest with invitation code validation
CREATE OR REPLACE FUNCTION update_guest_for_group(
  p_guest_id UUID,
  p_guest_group_id UUID,
  p_invitation_code UUID,
  p_name TEXT,
  p_attending_locations TEXT[],
  p_dietary_preferences JSONB,
  p_age_category TEXT DEFAULT 'adult'
)
RETURNS guests
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
  v_guest guests;
  v_group_locations TEXT[];
BEGIN
  -- Validate invitation code and fetch group locations
  SELECT locations INTO v_group_locations
  FROM guest_groups
  WHERE id = p_guest_group_id
  AND invitation_code = p_invitation_code;

  IF v_group_locations IS NULL THEN
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

  -- Reject NULL parameters (PostgreSQL three-valued logic would bypass subsequent checks)
  IF p_name IS NULL THEN
    RAISE EXCEPTION 'Guest name cannot be null';
  END IF;

  -- Normalize name by trimming whitespace
  p_name := trim(p_name);

  IF p_attending_locations IS NULL THEN
    RAISE EXCEPTION 'Attending locations cannot be null';
  END IF;

  -- Deduplicate attending_locations to prevent duplicate entries
  p_attending_locations := ARRAY(SELECT DISTINCT unnest(p_attending_locations));

  IF p_age_category IS NULL THEN
    RAISE EXCEPTION 'Age category cannot be null';
  END IF;

  -- Validate name
  IF length(trim(p_name)) = 0 THEN
    RAISE EXCEPTION 'Guest name cannot be empty';
  END IF;

  IF length(p_name) > 200 THEN
    RAISE EXCEPTION 'Guest name must be 200 characters or less';
  END IF;

  -- Validate dietary_preferences
  IF NOT validate_dietary_preferences(p_dietary_preferences) THEN
    RAISE EXCEPTION 'Invalid dietary preferences: must be a JSON object with keys (vegetarian, vegan, halal, no_pork, gluten_free as booleans; other as string up to 500 chars)';
  END IF;

  -- Validate age_category
  IF NOT (p_age_category = ANY(valid_age_categories())) THEN
    RAISE EXCEPTION 'Invalid age category. Must be one of: %', array_to_string(valid_age_categories(), ', ');
  END IF;

  -- Validate attending_locations against global valid locations
  IF NOT (p_attending_locations <@ valid_locations()) THEN
    RAISE EXCEPTION 'Invalid attending locations. Each location must be one of: %', array_to_string(valid_locations(), ', ');
  END IF;

  -- Validate attending_locations against group's invited locations
  IF NOT (p_attending_locations <@ v_group_locations) THEN
    RAISE EXCEPTION 'Attending locations must be within the group''s invited locations: %', array_to_string(v_group_locations, ', ');
  END IF;

  -- Update guest
  UPDATE guests
  SET name = p_name,
      attending_locations = p_attending_locations,
      dietary_preferences = p_dietary_preferences,
      age_category = p_age_category
  WHERE id = p_guest_id
  AND guest_group_id = p_guest_group_id
  RETURNING * INTO v_guest;

  RETURN v_guest;
END;
$$;

-- Delete a guest with invitation code validation
CREATE OR REPLACE FUNCTION delete_guest_for_group(
  p_guest_id UUID,
  p_guest_group_id UUID,
  p_invitation_code UUID
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
  WHERE id = p_guest_id
  AND guest_group_id = p_guest_group_id;

  RETURN TRUE;
END;
$$;

-- Revoke default PUBLIC execute, then grant only to intended roles
REVOKE EXECUTE ON FUNCTION get_guests_for_group(UUID, UUID) FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION create_guest_for_group(UUID, UUID, TEXT, TEXT[], JSONB, TEXT) FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION update_guest_for_group(UUID, UUID, UUID, TEXT, TEXT[], JSONB, TEXT) FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION delete_guest_for_group(UUID, UUID, UUID) FROM PUBLIC;

GRANT EXECUTE ON FUNCTION get_guests_for_group(UUID, UUID) TO anon, authenticated;
GRANT EXECUTE ON FUNCTION create_guest_for_group(UUID, UUID, TEXT, TEXT[], JSONB, TEXT) TO anon, authenticated;
GRANT EXECUTE ON FUNCTION update_guest_for_group(UUID, UUID, UUID, TEXT, TEXT[], JSONB, TEXT) TO anon, authenticated;
GRANT EXECUTE ON FUNCTION delete_guest_for_group(UUID, UUID, UUID) TO anon, authenticated;

-- ============================================================================
-- SECURITY - Row Level Security (RLS)
-- ============================================================================

-- Enable RLS on all tables
ALTER TABLE guest_groups ENABLE ROW LEVEL SECURITY;
ALTER TABLE guests ENABLE ROW LEVEL SECURITY;

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

-- Explicitly revoke all direct table access for anonymous users (defense-in-depth)
-- Anonymous users must use SECURITY DEFINER RPC functions with invitation_code validation
REVOKE ALL ON guest_groups FROM anon;
REVOKE ALL ON guests FROM anon;

-- Grant permissions only to authenticated admins
GRANT SELECT, INSERT, UPDATE, DELETE ON guest_groups TO authenticated;
GRANT SELECT, INSERT, UPDATE, DELETE ON guests TO authenticated;

-- ============================================================================
-- SECURE FUNCTION - Update Party Size with Invitation Code Validation
-- ============================================================================

-- Create a secure function to update party_size with invitation code validation
-- This allows guests to increase their party size when RSVPing without compromising security
CREATE OR REPLACE FUNCTION update_guest_group_party_size(
  p_guest_group_id UUID,
  p_invitation_code UUID,
  p_new_party_size INTEGER
)
RETURNS TABLE (
  id UUID,
  name TEXT,
  email TEXT,
  party_size INTEGER,
  locations TEXT[],
  default_language TEXT,
  additional_notes TEXT,
  created_at TIMESTAMP WITH TIME ZONE,
  updated_at TIMESTAMP WITH TIME ZONE
)
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
BEGIN
  -- Validate that the invitation code matches the guest_group_id
  IF NOT EXISTS (
    SELECT 1 FROM guest_groups
    WHERE guest_groups.id = p_guest_group_id
    AND guest_groups.invitation_code = p_invitation_code
  ) THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  -- Reject NULL to provide a clear error instead of a raw constraint violation
  IF p_new_party_size IS NULL THEN
    RAISE EXCEPTION 'Party size cannot be null';
  END IF;

  -- Validate party_size is reasonable (between 1 and 20)
  IF p_new_party_size < 1 OR p_new_party_size > 20 THEN
    RAISE EXCEPTION 'Party size must be between 1 and 20';
  END IF;

  -- Update the party_size and return explicit columns (excluding invitation_code)
  RETURN QUERY
  UPDATE guest_groups
  SET party_size = p_new_party_size
  WHERE guest_groups.id = p_guest_group_id
  RETURNING
    guest_groups.id,
    guest_groups.name,
    guest_groups.email,
    guest_groups.party_size,
    guest_groups.locations,
    guest_groups.default_language,
    guest_groups.additional_notes,
    guest_groups.created_at,
    guest_groups.updated_at;
END;
$$;

-- Revoke default PUBLIC execute, then grant only to intended roles
REVOKE EXECUTE ON FUNCTION update_guest_group_party_size(UUID, UUID, INTEGER) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION update_guest_group_party_size(UUID, UUID, INTEGER) TO anon, authenticated;

COMMENT ON FUNCTION update_guest_group_party_size(UUID, UUID, INTEGER) IS
  'Securely updates party_size for a guest_group by validating the invitation code. Returns explicit columns excluding invitation_code.';

-- Update additional_notes for a guest group with invitation code validation
CREATE OR REPLACE FUNCTION update_guest_group_notes(
  p_guest_group_id UUID,
  p_invitation_code UUID,
  p_additional_notes TEXT
)
RETURNS TABLE (
  id UUID,
  name TEXT,
  email TEXT,
  party_size INTEGER,
  locations TEXT[],
  default_language TEXT,
  additional_notes TEXT,
  created_at TIMESTAMP WITH TIME ZONE,
  updated_at TIMESTAMP WITH TIME ZONE
)
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
BEGIN
  -- Validate invitation code
  IF NOT EXISTS (
    SELECT 1 FROM guest_groups
    WHERE guest_groups.id = p_guest_group_id
    AND guest_groups.invitation_code = p_invitation_code
  ) THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  -- Validate notes length
  IF p_additional_notes IS NOT NULL AND length(p_additional_notes) > 2000 THEN
    RAISE EXCEPTION 'Additional notes must be 2000 characters or less';
  END IF;

  -- Update the additional_notes and return explicit columns (excluding invitation_code)
  RETURN QUERY
  UPDATE guest_groups
  SET additional_notes = p_additional_notes
  WHERE guest_groups.id = p_guest_group_id
  RETURNING
    guest_groups.id,
    guest_groups.name,
    guest_groups.email,
    guest_groups.party_size,
    guest_groups.locations,
    guest_groups.default_language,
    guest_groups.additional_notes,
    guest_groups.created_at,
    guest_groups.updated_at;
END;
$$;

REVOKE EXECUTE ON FUNCTION update_guest_group_notes(UUID, UUID, TEXT) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION update_guest_group_notes(UUID, UUID, TEXT) TO anon, authenticated;

-- ============================================================================
-- GUEST LOCATION ATTENDANCE FUNCTIONS
-- ============================================================================

-- Set guest attendance for a location (with invitation code validation)
CREATE OR REPLACE FUNCTION set_guest_location_attendance(
  p_guest_id UUID,
  p_guest_group_id UUID,
  p_invitation_code UUID,
  p_location TEXT,
  p_attending BOOLEAN
)
RETURNS void
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
  v_group_locations TEXT[];
BEGIN
  -- Validate invitation code and fetch group locations
  SELECT locations INTO v_group_locations
  FROM guest_groups
  WHERE id = p_guest_group_id
  AND invitation_code = p_invitation_code;

  IF v_group_locations IS NULL THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  -- Reject NULL parameters
  IF p_location IS NULL THEN
    RAISE EXCEPTION 'Location cannot be null';
  END IF;

  IF p_attending IS NULL THEN
    RAISE EXCEPTION 'Attending flag cannot be null';
  END IF;

  -- Validate guest belongs to the group
  IF NOT EXISTS (
    SELECT 1 FROM guests
    WHERE id = p_guest_id
    AND guest_group_id = p_guest_group_id
  ) THEN
    RAISE EXCEPTION 'Guest does not belong to this group';
  END IF;

  -- Validate location
  IF NOT (p_location = ANY(valid_locations())) THEN
    RAISE EXCEPTION 'Invalid location. Must be one of: %', array_to_string(valid_locations(), ', ');
  END IF;

  -- Validate location is within the group's invited locations
  IF NOT (p_location = ANY(v_group_locations)) THEN
    RAISE EXCEPTION 'Location ''%'' is not available for this group. Group locations: %', p_location, array_to_string(v_group_locations, ', ');
  END IF;

  IF p_attending THEN
    -- Add location to array if not already present
    UPDATE guests
    SET attending_locations = array_append(attending_locations, p_location)
    WHERE id = p_guest_id
    AND guest_group_id = p_guest_group_id
    AND NOT (p_location = ANY(attending_locations));
  ELSE
    -- Remove location from array
    UPDATE guests
    SET attending_locations = array_remove(attending_locations, p_location)
    WHERE id = p_guest_id
    AND guest_group_id = p_guest_group_id;
  END IF;
END;
$$;

-- Get attending guests for a location (with invitation code validation)
CREATE OR REPLACE FUNCTION get_attending_guests_for_location(
  p_guest_group_id UUID,
  p_invitation_code UUID,
  p_location TEXT
)
RETURNS TABLE (
  guest_id UUID,
  guest_name TEXT,
  dietary_preferences JSONB
)
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
  v_group_locations TEXT[];
BEGIN
  -- Validate invitation code and fetch group locations
  SELECT locations INTO v_group_locations
  FROM guest_groups
  WHERE id = p_guest_group_id
  AND invitation_code = p_invitation_code;

  IF v_group_locations IS NULL THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  -- Reject NULL parameter
  IF p_location IS NULL THEN
    RAISE EXCEPTION 'Location cannot be null';
  END IF;

  -- Validate location
  IF NOT (p_location = ANY(valid_locations())) THEN
    RAISE EXCEPTION 'Invalid location. Must be one of: %', array_to_string(valid_locations(), ', ');
  END IF;

  -- Validate location is within the group's invited locations
  IF NOT (p_location = ANY(v_group_locations)) THEN
    RAISE EXCEPTION 'Location ''%'' is not available for this group. Group locations: %', p_location, array_to_string(v_group_locations, ', ');
  END IF;

  RETURN QUERY
  SELECT
    g.id as guest_id,
    g.name as guest_name,
    g.dietary_preferences
  FROM guests g
  WHERE g.guest_group_id = p_guest_group_id
  AND p_location = ANY(g.attending_locations)
  ORDER BY g.name;
END;
$$;

-- Bulk update guest attendance for a location (with invitation code validation)
CREATE OR REPLACE FUNCTION bulk_update_guest_location_attendance(
  p_guest_group_id UUID,
  p_invitation_code UUID,
  p_location TEXT,
  p_guest_ids UUID[]
)
RETURNS void
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
  v_group_locations TEXT[];
BEGIN
  -- Validate invitation code and fetch group locations
  SELECT locations INTO v_group_locations
  FROM guest_groups
  WHERE id = p_guest_group_id
  AND invitation_code = p_invitation_code;

  IF v_group_locations IS NULL THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  -- Reject NULL parameters
  IF p_location IS NULL THEN
    RAISE EXCEPTION 'Location cannot be null';
  END IF;

  IF p_guest_ids IS NULL THEN
    RAISE EXCEPTION 'Guest IDs array cannot be null';
  END IF;

  -- Validate guest_ids array size (matches max 20 guests per group limit)
  IF array_length(p_guest_ids, 1) > 20 THEN
    RAISE EXCEPTION 'Cannot update more than 20 guests at once';
  END IF;

  -- Validate location
  IF NOT (p_location = ANY(valid_locations())) THEN
    RAISE EXCEPTION 'Invalid location. Must be one of: %', array_to_string(valid_locations(), ', ');
  END IF;

  -- Validate location is within the group's invited locations
  IF NOT (p_location = ANY(v_group_locations)) THEN
    RAISE EXCEPTION 'Location ''%'' is not available for this group. Group locations: %', p_location, array_to_string(v_group_locations, ', ');
  END IF;

  -- Remove location from all guests in this group
  UPDATE guests
  SET attending_locations = array_remove(attending_locations, p_location)
  WHERE guest_group_id = p_guest_group_id;

  -- Add location to specified guests
  UPDATE guests
  SET attending_locations = array_append(attending_locations, p_location)
  WHERE id = ANY(p_guest_ids)
  AND guest_group_id = p_guest_group_id
  AND NOT (p_location = ANY(attending_locations));
END;
$$;

-- Revoke default PUBLIC execute, then grant only to intended roles
REVOKE EXECUTE ON FUNCTION set_guest_location_attendance(UUID, UUID, UUID, TEXT, BOOLEAN) FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION get_attending_guests_for_location(UUID, UUID, TEXT) FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION bulk_update_guest_location_attendance(UUID, UUID, TEXT, UUID[]) FROM PUBLIC;

GRANT EXECUTE ON FUNCTION set_guest_location_attendance(UUID, UUID, UUID, TEXT, BOOLEAN) TO anon, authenticated;
GRANT EXECUTE ON FUNCTION get_attending_guests_for_location(UUID, UUID, TEXT) TO anon, authenticated;
GRANT EXECUTE ON FUNCTION bulk_update_guest_location_attendance(UUID, UUID, TEXT, UUID[]) TO anon, authenticated;

-- ============================================================================
-- VIEWS - Statistics and Reports
-- ============================================================================





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
  RAISE NOTICE '  • guests (individual invitees with location attendance)';
  RAISE NOTICE '';
  RAISE NOTICE '🔒 Security Features:';
  RAISE NOTICE '  ✓ Row Level Security (RLS) enabled on all tables';
  RAISE NOTICE '  ✓ Guest group isolation via SECURITY DEFINER functions';
  RAISE NOTICE '  ✓ Invitation code validation on all guest operations';
  RAISE NOTICE '  ✓ Admin full access via JWT authentication';
  RAISE NOTICE '  ✓ Anonymous users must use RPC functions with invitation_code';
  RAISE NOTICE '';

  RAISE NOTICE '📝 Table Naming:';
  RAISE NOTICE '  • guest_groups = invitation groups (formerly "guests")';
  RAISE NOTICE '  • guests = individual invitees (formerly "invitees")';
  RAISE NOTICE '';
  RAISE NOTICE '📝 Next Steps:';
  RAISE NOTICE '  1. Create admin user: Dashboard → Authentication → Users';
  RAISE NOTICE '  2. Add guest groups via admin panel';
  RAISE NOTICE '  3. Frontend MUST use RPC functions for guest/RSVP operations:';
  RAISE NOTICE '     • authenticate_guest_group(invitation_code)';
  RAISE NOTICE '     • get_guests_for_group(guest_group_id, invitation_code)';
  RAISE NOTICE '     • create_guest_for_group(guest_group_id, invitation_code, name, ...)';
  RAISE NOTICE '     • update_guest_for_group(guest_id, guest_group_id, invitation_code, ...)';
  RAISE NOTICE '     • delete_guest_for_group(guest_id, guest_group_id, invitation_code)';
  RAISE NOTICE '     • update_guest_group_party_size(guest_group_id, invitation_code, size)';
  RAISE NOTICE '     • update_guest_group_notes(guest_group_id, invitation_code, notes)';
  RAISE NOTICE '     • set_guest_location_attendance(guest_id, group_id, code, location, bool)';
  RAISE NOTICE '     • get_attending_guests_for_location(group_id, code, location)';
  RAISE NOTICE '     • bulk_update_guest_location_attendance(group_id, code, location, ids)';
  RAISE NOTICE '  ⚠️  WARNING: Direct table queries for guests will FAIL for anon users';
  RAISE NOTICE '';
  RAISE NOTICE '============================================================================';
END $$;
