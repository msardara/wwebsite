-- Convert invited_by from TEXT[] to JSONB
-- Drop the old default first, retype, then restore the new default

ALTER TABLE guest_groups
    ALTER COLUMN invited_by DROP DEFAULT;

ALTER TABLE guest_groups
    ALTER COLUMN invited_by TYPE JSONB USING to_jsonb(invited_by);

ALTER TABLE guest_groups
    ALTER COLUMN invited_by SET DEFAULT '["mauro.sardara@gmail.com"]';
