-- Add self_added column to distinguish admin-created guests from guest-added ones.
-- Admin-created guests must not be deletable by guests through the RSVP page.
ALTER TABLE guests
  ADD COLUMN self_added BOOLEAN NOT NULL DEFAULT FALSE;

-- All existing guests were admin-created — no backfill needed (DEFAULT FALSE covers it).

-- Re-create save_rsvp to:
--   1. Set self_added = TRUE for newly inserted guests (those with temp_ IDs)
--   2. Only delete self_added = TRUE guests not present in the submitted list
--      (admin-created guests are never deleted via the RSVP flow)
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
  v_group_locations     TEXT[];
  v_guest_obj           JSONB;
  v_guest_id            UUID;
  v_name                TEXT;
  v_attending_locations TEXT[];
  v_dietary_preferences JSONB;
  v_age_category        TEXT;
  v_guest               guests;
  v_new_party_size      INTEGER;
  v_current_party_size  INTEGER;
  v_submitted_ids       UUID[] := ARRAY[]::UUID[];
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
    RAISE EXCEPTION 'Cannot submit more than 20 guests at once';
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
      -- Update existing guest (admin- or self-added)
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

      v_submitted_ids := array_append(v_submitted_ids, v_guest_id);
      RETURN NEXT v_guest;
    ELSE
      -- Create new guest — always self_added = TRUE
      INSERT INTO guests (
        guest_group_id, name,
        attending_locations,
        dietary_preferences, age_category,
        self_added
      )
      VALUES (
        p_guest_group_id, v_name,
        v_attending_locations,
        v_dietary_preferences, v_age_category,
        TRUE
      )
      RETURNING * INTO v_guest;

      v_submitted_ids := array_append(v_submitted_ids, v_guest.id);
      RETURN NEXT v_guest;
    END IF;
  END LOOP;

  -- Only remove self-added guests that were not submitted.
  -- Admin-created guests (self_added = FALSE) are never deleted via RSVP.
  DELETE FROM guests
  WHERE guest_group_id = p_guest_group_id
    AND self_added = TRUE
    AND id != ALL(v_submitted_ids);

  -- Update party_size
  IF v_new_party_size > 20 THEN
    RAISE EXCEPTION 'Party size must be between 1 and 20';
  END IF;

  UPDATE guest_groups
  SET party_size = v_new_party_size
  WHERE id = p_guest_group_id;

  -- Update additional notes
  IF p_additional_notes IS NOT NULL
    AND length(p_additional_notes) > 2000
  THEN
    RAISE EXCEPTION 'Additional notes must be 2000 characters or less';
  END IF;

  UPDATE guest_groups
  SET additional_notes = p_additional_notes
  WHERE id = p_guest_group_id;

  -- Mark group as having submitted their RSVP
  UPDATE guest_groups
  SET rsvp_submitted = TRUE
  WHERE id = p_guest_group_id;

  RETURN;
END;
$$;
