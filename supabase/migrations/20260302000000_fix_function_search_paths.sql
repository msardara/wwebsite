-- Fix mutable search_path warnings for non-SECURITY DEFINER functions.
-- All functions must declare an explicit search_path to prevent
-- search-path injection attacks.

CREATE OR REPLACE FUNCTION public.valid_locations()
RETURNS TEXT[]
LANGUAGE sql
IMMUTABLE
SET search_path = public
AS $$
  SELECT ARRAY['sardinia', 'tunisia', 'nice']::text[];
$$;

CREATE OR REPLACE FUNCTION public.valid_age_categories()
RETURNS TEXT[]
LANGUAGE sql
IMMUTABLE
SET search_path = public
AS $$
  SELECT ARRAY[
    'adult', 'child_under_3', 'child_under_10'
  ]::text[];
$$;

CREATE OR REPLACE FUNCTION public.array_has_no_duplicates(arr TEXT[])
RETURNS BOOLEAN
LANGUAGE sql
IMMUTABLE
SET search_path = public
AS $$
  SELECT cardinality(arr) = cardinality(
    ARRAY(SELECT DISTINCT unnest(arr))
  );
$$;

CREATE OR REPLACE FUNCTION public.validate_dietary_preferences(
  prefs JSONB
)
RETURNS BOOLEAN
LANGUAGE plpgsql
IMMUTABLE
SET search_path = public
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

CREATE OR REPLACE FUNCTION public.validate_and_normalize_guest_fields(
  INOUT p_name TEXT,
  INOUT p_attending_locations TEXT[],
  p_dietary_preferences JSONB,
  p_age_category TEXT,
  p_group_locations TEXT[]
)
LANGUAGE plpgsql
IMMUTABLE
SET search_path = public
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
