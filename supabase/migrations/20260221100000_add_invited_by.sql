-- Add invited_by column to guest_groups
-- Stores the list of admin emails who invited this group
-- Existing rows are backfilled with mauro.sardara@gmail.com

ALTER TABLE guest_groups
    ADD COLUMN IF NOT EXISTS invited_by JSONB NOT NULL DEFAULT '["mauro.sardara@gmail.com"]';
