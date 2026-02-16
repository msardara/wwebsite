-- Wedding Website - Initial Database Schema
-- Tables: guest_groups, guests


-- ====================================================================
-- SECTION 1: DOMAIN CONSTANTS & VALIDATION FUNCTIONS
-- ====================================================================

-- Valid wedding locations
CREATE OR REPLACE FUNCTION valid_locations()
RETURNS TEXT[]
LANGUAGE sql
IMMUTABLE
AS $$
  SELECT ARRAY['sardinia', 'tunisia', 'nice']::text[];
$$;

-- Valid age categories
CREATE OR REPLACE FUNCTION valid_age_categories()
RETURNS TEXT[]
LANGUAGE sql
IMMUTABLE
AS $$
  SELECT ARRAY[
    'adult', 'child_under_3', 'child_under_10'
  ]::text[];
$$;

-- Check that a TEXT array contains no duplicate values
CREATE OR REPLACE FUNCTION array_has_no_duplicates(arr TEXT[])
RETURNS BOOLEAN
LANGUAGE sql
IMMUTABLE
AS $$
  SELECT cardinality(arr) = cardinality(
    ARRAY(SELECT DISTINCT unnest(arr))
  );
$$;

-- Validate dietary_preferences JSONB structure
CREATE OR REPLACE FUNCTION validate_dietary_preferences(
  prefs JSONB
)
RETURNS BOOLEAN
LANGUAGE plpgsql
IMMUTABLE
AS $$
DECLARE
  allowed_keys TEXT[] := ARRAY[
    'vegetarian', 'vegan', 'halal',
    'no_pork', 'gluten_free', 'other'
  ];
  key TEXT;
BEGIN
  IF prefs IS NULL THEN
    RETURN TRUE;
  END IF;

  IF jsonb_typeof(prefs) != 'object' THEN
    RETURN FALSE;
  END IF;

  FOR key IN SELECT jsonb_object_keys(prefs)
  LOOP
    IF NOT (key = ANY(allowed_keys)) THEN
      RETURN FALSE;
    END IF;
  END LOOP;

  -- Boolean fields
  IF prefs ? 'vegetarian'
    AND jsonb_typeof(prefs -> 'vegetarian') != 'boolean'
  THEN RETURN FALSE; END IF;

  IF prefs ? 'vegan'
    AND jsonb_typeof(prefs -> 'vegan') != 'boolean'
  THEN RETURN FALSE; END IF;

  IF prefs ? 'halal'
    AND jsonb_typeof(prefs -> 'halal') != 'boolean'
  THEN RETURN FALSE; END IF;

  IF prefs ? 'no_pork'
    AND jsonb_typeof(prefs -> 'no_pork') != 'boolean'
  THEN RETURN FALSE; END IF;

  IF prefs ? 'gluten_free'
    AND jsonb_typeof(prefs -> 'gluten_free') != 'boolean'
  THEN RETURN FALSE; END IF;

  -- 'other' must be a string, max 500 characters
  IF prefs ? 'other' THEN
    IF jsonb_typeof(prefs -> 'other') != 'string'
    THEN RETURN FALSE; END IF;

    IF length(prefs ->> 'other') > 500
    THEN RETURN FALSE; END IF;
  END IF;

  -- Total serialized size limit
  IF length(prefs::text) > 1000 THEN
    RETURN FALSE;
  END IF;

  RETURN TRUE;
END;
$$;


-- ====================================================================
-- SECTION 2: TABLES & INDEXES
-- ====================================================================

CREATE TABLE guest_groups (
  id              UUID PRIMARY KEY
                    DEFAULT gen_random_uuid(),
  name            TEXT NOT NULL,
  email           TEXT,
  invitation_code UUID UNIQUE NOT NULL
                    DEFAULT gen_random_uuid(),
  party_size      INTEGER NOT NULL DEFAULT 1
                    CHECK (party_size > 0 AND party_size <= 20),
  locations       TEXT[] NOT NULL,
  default_language TEXT NOT NULL DEFAULT 'en'
                    CHECK (default_language IN ('en', 'fr', 'it')),
  additional_notes TEXT,
  created_at      TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at      TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

  CONSTRAINT guest_groups_name_not_empty
    CHECK (length(trim(name)) > 0),
  CONSTRAINT guest_groups_name_length
    CHECK (length(name) <= 200),
  CONSTRAINT guest_groups_email_length
    CHECK (email IS NULL OR length(email) <= 254),
  CONSTRAINT guest_groups_additional_notes_length
    CHECK (additional_notes IS NULL
      OR length(additional_notes) <= 2000),
  CONSTRAINT guest_groups_locations_valid
    CHECK (locations <@ valid_locations()),
  CONSTRAINT guest_groups_locations_not_empty
    CHECK (cardinality(locations) > 0),
  CONSTRAINT guest_groups_locations_no_duplicates
    CHECK (array_has_no_duplicates(locations))
);

-- invitation_code already has an implicit unique index
CREATE INDEX idx_guest_groups_locations
  ON guest_groups USING GIN(locations);
CREATE INDEX idx_guest_groups_default_language
  ON guest_groups(default_language);

CREATE TABLE guests (
  id                   UUID PRIMARY KEY
                         DEFAULT gen_random_uuid(),
  guest_group_id       UUID NOT NULL
                         REFERENCES guest_groups(id)
                         ON DELETE CASCADE,
  name                 TEXT NOT NULL,
  attending_locations  TEXT[] NOT NULL DEFAULT '{}',
  dietary_preferences  JSONB DEFAULT '{
    "vegetarian": false,
    "vegan": false,
    "halal": false,
    "no_pork": false,
    "gluten_free": false,
    "other": ""
  }'::jsonb,
  age_category         TEXT DEFAULT 'adult'
                         CHECK (age_category = ANY(
                           valid_age_categories()
                         )),
  created_at           TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at           TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

  CONSTRAINT guests_name_not_empty
    CHECK (length(trim(name)) > 0),
  CONSTRAINT guests_name_length
    CHECK (length(name) <= 200),
  CONSTRAINT guests_dietary_valid
    CHECK (validate_dietary_preferences(dietary_preferences)),
  CONSTRAINT guests_attending_locations_valid
    CHECK (attending_locations <@ valid_locations()),
  CONSTRAINT guests_attending_locations_no_duplicates
    CHECK (array_has_no_duplicates(attending_locations))
);

CREATE INDEX idx_guests_guest_group_id
  ON guests(guest_group_id);
CREATE INDEX idx_guests_attending_locations
  ON guests USING GIN(attending_locations);
CREATE INDEX idx_guests_age_category
  ON guests(age_category);


-- ====================================================================
-- SECTION 3: SHARED COMPOSITE TYPE
-- ====================================================================

-- Public projection of guest_groups (excludes invitation_code)
CREATE TYPE guest_group_public_info AS (
  id               UUID,
  name             TEXT,
  email            TEXT,
  party_size       INTEGER,
  locations        TEXT[],
  default_language TEXT,
  additional_notes TEXT,
  created_at       TIMESTAMP WITH TIME ZONE,
  updated_at       TIMESTAMP WITH TIME ZONE
);


-- ====================================================================
-- SECTION 4: TRIGGER FUNCTIONS
-- ====================================================================

-- Auto-update updated_at on row modification
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

CREATE TRIGGER update_guest_groups_updated_at
  BEFORE UPDATE ON guest_groups
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_guests_updated_at
  BEFORE UPDATE ON guests
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

REVOKE EXECUTE
  ON FUNCTION update_updated_at_column()
  FROM PUBLIC;

-- Prevent modification of invitation_code after creation
CREATE OR REPLACE FUNCTION prevent_invitation_code_update()
RETURNS TRIGGER
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
BEGIN
  IF OLD.invitation_code IS DISTINCT FROM NEW.invitation_code
  THEN
    RAISE EXCEPTION
      'invitation_code cannot be modified after creation';
  END IF;
  RETURN NEW;
END;
$$;

CREATE TRIGGER enforce_invitation_code_immutable
  BEFORE UPDATE ON guest_groups
  FOR EACH ROW
  EXECUTE FUNCTION prevent_invitation_code_update();

REVOKE EXECUTE
  ON FUNCTION prevent_invitation_code_update()
  FROM PUBLIC;


-- ====================================================================
-- SECTION 5: AUTHORIZATION & VALIDATION HELPERS
-- ====================================================================

-- Verify (group_id, invitation_code); return group locations
CREATE OR REPLACE FUNCTION
  validate_invitation_code_and_get_locations(
    p_guest_group_id UUID,
    p_invitation_code UUID
  )
RETURNS TEXT[]
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
  v_group_locations TEXT[];
BEGIN
  SELECT locations INTO v_group_locations
  FROM guest_groups
  WHERE id = p_guest_group_id
    AND invitation_code = p_invitation_code;

  IF v_group_locations IS NULL THEN
    RAISE EXCEPTION 'Invalid guest group or invitation code';
  END IF;

  RETURN v_group_locations;
END;
$$;

REVOKE EXECUTE
  ON FUNCTION validate_invitation_code_and_get_locations(
    UUID, UUID
  )
  FROM PUBLIC;

-- Verify that a guest belongs to the specified group
CREATE OR REPLACE FUNCTION validate_guest_membership(
  p_guest_id UUID,
  p_guest_group_id UUID
)
RETURNS VOID
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
BEGIN
  IF NOT EXISTS (
    SELECT 1 FROM guests
    WHERE id = p_guest_id
      AND guest_group_id = p_guest_group_id
  ) THEN
    RAISE EXCEPTION 'Guest does not belong to this group';
  END IF;
END;
$$;

REVOKE EXECUTE
  ON FUNCTION validate_guest_membership(UUID, UUID)
  FROM PUBLIC;

-- Validate and normalize mutable guest fields
CREATE OR REPLACE FUNCTION validate_and_normalize_guest_fields(
  INOUT p_name TEXT,
  INOUT p_attending_locations TEXT[],
  p_dietary_preferences JSONB,
  p_age_category TEXT,
  p_group_locations TEXT[]
)
LANGUAGE plpgsql
IMMUTABLE
AS $$
BEGIN
  -- NULL guards
  IF p_name IS NULL THEN
    RAISE EXCEPTION 'Guest name cannot be null';
  END IF;
  IF p_attending_locations IS NULL THEN
    RAISE EXCEPTION 'Attending locations cannot be null';
  END IF;
  IF p_age_category IS NULL THEN
    RAISE EXCEPTION 'Age category cannot be null';
  END IF;

  -- Normalize name
  p_name := trim(p_name);

  IF length(p_name) = 0 THEN
    RAISE EXCEPTION 'Guest name cannot be empty';
  END IF;
  IF length(p_name) > 200 THEN
    RAISE EXCEPTION
      'Guest name must be 200 characters or less';
  END IF;

  -- Deduplicate attending_locations
  p_attending_locations := ARRAY(
    SELECT DISTINCT unnest(p_attending_locations)
  );

  -- Validate dietary_preferences
  IF NOT validate_dietary_preferences(
    p_dietary_preferences
  ) THEN
    RAISE EXCEPTION
      'Invalid dietary preferences: must be a JSON object '
      'with keys (vegetarian, vegan, halal, no_pork, '
      'gluten_free as booleans; other as string '
      'up to 500 chars)';
  END IF;

  -- Validate age_category
  IF NOT (p_age_category = ANY(valid_age_categories()))
  THEN
    RAISE EXCEPTION
      'Invalid age category. Must be one of: %',
      array_to_string(valid_age_categories(), ', ');
  END IF;

  -- Validate against global valid locations
  IF NOT (p_attending_locations <@ valid_locations())
  THEN
    RAISE EXCEPTION
      'Invalid attending locations. Must be one of: %',
      array_to_string(valid_locations(), ', ');
  END IF;

  -- Validate against group's invited locations
  IF NOT (p_attending_locations <@ p_group_locations)
  THEN
    RAISE EXCEPTION
      'Attending locations must be within the '
      'group''s invited locations: %',
      array_to_string(p_group_locations, ', ');
  END IF;
END;
$$;

-- Build a guest_group_public_info from a guest_groups row
CREATE OR REPLACE FUNCTION project_guest_group_public_info(
  p_guest_group_id UUID
)
RETURNS SETOF guest_group_public_info
LANGUAGE sql
SECURITY DEFINER
SET search_path = public
AS $$
  SELECT
    gg.id,
    gg.name,
    gg.email,
    gg.party_size,
    gg.locations,
    gg.default_language,
    gg.additional_notes,
    gg.created_at,
    gg.updated_at
  FROM guest_groups gg
  WHERE gg.id = p_guest_group_id;
$$;

REVOKE EXECUTE
  ON FUNCTION project_guest_group_public_info(UUID)
  FROM PUBLIC;


-- ====================================================================
-- SECTION 6: BUSINESS LOGIC / RPC FUNCTIONS
-- ====================================================================

-- Authenticate a guest group by invitation code
CREATE OR REPLACE FUNCTION authenticate_guest_group(
  code UUID
)
RETURNS SETOF guest_group_public_info
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
    gg.additional_notes,
    gg.created_at,
    gg.updated_at
  FROM guest_groups gg
  WHERE gg.invitation_code = code;
END;
$$;

-- Fetch all guests for an authenticated group
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
  PERFORM validate_invitation_code_and_get_locations(
    p_guest_group_id, p_invitation_code
  );

  RETURN QUERY
  SELECT * FROM guests
  WHERE guest_group_id = p_guest_group_id
  ORDER BY created_at;
END;
$$;

-- Bulk RSVP save: create/update/delete guests and update
-- group notes and party_size in a single transaction.
-- The submitted guest list is the source of truth;
-- absent guests are removed.
CREATE OR REPLACE FUNCTION save_rsvp(
  p_guest_group_id   UUID,
  p_invitation_code  UUID,
  p_guests           JSONB,
  p_additional_notes TEXT DEFAULT NULL
)
RETURNS SETOF guests
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = public
AS $$
DECLARE
  v_group_locations    TEXT[];
  v_guest_obj          JSONB;
  v_guest_id           UUID;
  v_name               TEXT;
  v_attending_locations TEXT[];
  v_dietary_preferences JSONB;
  v_age_category       TEXT;
  v_guest              guests;
  v_new_party_size     INTEGER;
  v_current_party_size INTEGER;
  v_submitted_ids      UUID[] := ARRAY[]::UUID[];
BEGIN
  -- Authenticate and retrieve group locations
  v_group_locations :=
    validate_invitation_code_and_get_locations(
      p_guest_group_id, p_invitation_code
    );

  IF p_guests IS NULL
    OR jsonb_typeof(p_guests) != 'array'
  THEN
    RAISE EXCEPTION 'p_guests must be a JSON array';
  END IF;

  v_new_party_size := jsonb_array_length(p_guests);

  IF v_new_party_size < 1 THEN
    RAISE EXCEPTION 'At least one guest is required';
  END IF;

  IF v_new_party_size > 20 THEN
    RAISE EXCEPTION
      'Cannot submit more than 20 guests at once';
  END IF;

  -- Lock group row
  SELECT gg.party_size INTO v_current_party_size
  FROM guest_groups gg
  WHERE gg.id = p_guest_group_id
  FOR UPDATE;

  -- Process each guest: create or update
  FOR v_guest_obj IN
    SELECT * FROM jsonb_array_elements(p_guests)
  LOOP
    v_name := v_guest_obj ->> 'name';
    v_age_category := COALESCE(
      v_guest_obj ->> 'age_category', 'adult'
    );
    v_dietary_preferences := COALESCE(
      v_guest_obj -> 'dietary_preferences', '{}'::JSONB
    );

    SELECT COALESCE(
      array_agg(elem::TEXT), ARRAY[]::TEXT[]
    )
    INTO v_attending_locations
    FROM jsonb_array_elements_text(
      COALESCE(
        v_guest_obj -> 'attending_locations',
        '[]'::JSONB
      )
    ) AS elem;

    -- Validate and normalize
    SELECT f.p_name, f.p_attending_locations
    INTO v_name, v_attending_locations
    FROM validate_and_normalize_guest_fields(
      v_name, v_attending_locations,
      v_dietary_preferences, v_age_category,
      v_group_locations
    ) f;

    -- Parse guest id; non-UUID values (e.g. "temp_xxx")
    -- are treated as new guests
    v_guest_id := NULL;
    BEGIN
      v_guest_id := (v_guest_obj ->> 'id')::UUID;
    EXCEPTION WHEN OTHERS THEN
      v_guest_id := NULL;
    END;

    IF v_guest_id IS NOT NULL THEN
      -- Update existing guest
      PERFORM validate_guest_membership(
        v_guest_id, p_guest_group_id
      );

      UPDATE guests
      SET name                = v_name,
          attending_locations  = v_attending_locations,
          dietary_preferences  = v_dietary_preferences,
          age_category         = v_age_category
      WHERE id = v_guest_id
        AND guest_group_id = p_guest_group_id
      RETURNING * INTO v_guest;

      v_submitted_ids := array_append(
        v_submitted_ids, v_guest_id
      );
      RETURN NEXT v_guest;
    ELSE
      -- Create new guest
      INSERT INTO guests (
        guest_group_id, name,
        attending_locations,
        dietary_preferences, age_category
      )
      VALUES (
        p_guest_group_id, v_name,
        v_attending_locations,
        v_dietary_preferences, v_age_category
      )
      RETURNING * INTO v_guest;

      v_submitted_ids := array_append(
        v_submitted_ids, v_guest.id
      );
      RETURN NEXT v_guest;
    END IF;
  END LOOP;

  -- Remove guests not in the submitted list
  DELETE FROM guests
  WHERE guest_group_id = p_guest_group_id
    AND id != ALL(v_submitted_ids);

  -- Update party_size
  IF v_new_party_size > 20 THEN
    RAISE EXCEPTION
      'Party size must be between 1 and 20';
  END IF;

  UPDATE guest_groups
  SET party_size = v_new_party_size
  WHERE id = p_guest_group_id;

  -- Update additional notes
  IF p_additional_notes IS NOT NULL
    AND length(p_additional_notes) > 2000
  THEN
    RAISE EXCEPTION
      'Additional notes must be 2000 characters or less';
  END IF;

  UPDATE guest_groups
  SET additional_notes = p_additional_notes
  WHERE id = p_guest_group_id;

  RETURN;
END;
$$;


-- ====================================================================
-- SECTION 7: PRIVILEGE GRANTS
-- ====================================================================

REVOKE EXECUTE ON FUNCTION authenticate_guest_group(UUID)
  FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION get_guests_for_group(UUID, UUID)
  FROM PUBLIC;
REVOKE EXECUTE ON FUNCTION save_rsvp(UUID, UUID, JSONB, TEXT)
  FROM PUBLIC;

GRANT EXECUTE ON FUNCTION authenticate_guest_group(UUID)
  TO anon, authenticated;
GRANT EXECUTE ON FUNCTION get_guests_for_group(UUID, UUID)
  TO anon, authenticated;
GRANT EXECUTE ON FUNCTION save_rsvp(UUID, UUID, JSONB, TEXT)
  TO anon, authenticated;


-- ====================================================================
-- SECTION 8: ROW LEVEL SECURITY
-- ====================================================================

ALTER TABLE guest_groups ENABLE ROW LEVEL SECURITY;
ALTER TABLE guests ENABLE ROW LEVEL SECURITY;

-- guest_groups: admin-only direct access

CREATE POLICY "guest_groups_select_admin_only"
  ON guest_groups FOR SELECT
  TO authenticated
  USING (auth.role() = 'authenticated');

CREATE POLICY "guest_groups_insert_admin"
  ON guest_groups FOR INSERT
  TO authenticated
  WITH CHECK (auth.role() = 'authenticated');

CREATE POLICY "guest_groups_update_admin"
  ON guest_groups FOR UPDATE
  TO authenticated
  USING (auth.role() = 'authenticated')
  WITH CHECK (auth.role() = 'authenticated');

CREATE POLICY "guest_groups_delete_admin"
  ON guest_groups FOR DELETE
  TO authenticated
  USING (auth.role() = 'authenticated');

-- guests: admin-only direct access

CREATE POLICY "admins_can_view_all_guests"
  ON guests FOR SELECT
  TO authenticated
  USING (auth.role() = 'authenticated');

CREATE POLICY "admins_can_insert_guests"
  ON guests FOR INSERT
  TO authenticated
  WITH CHECK (auth.role() = 'authenticated');

CREATE POLICY "admins_can_update_guests"
  ON guests FOR UPDATE
  TO authenticated
  USING (auth.role() = 'authenticated')
  WITH CHECK (auth.role() = 'authenticated');

CREATE POLICY "admins_can_delete_guests"
  ON guests FOR DELETE
  TO authenticated
  USING (auth.role() = 'authenticated');

-- Anonymous users must use RPC functions
REVOKE ALL ON guest_groups FROM anon;
REVOKE ALL ON guests FROM anon;

GRANT SELECT, INSERT, UPDATE, DELETE
  ON guest_groups TO authenticated;
GRANT SELECT, INSERT, UPDATE, DELETE
  ON guests TO authenticated;


DO $$
BEGIN
  RAISE NOTICE
    'Schema created: guest_groups, guests, '
    'RPC functions, RLS policies.';
END $$;