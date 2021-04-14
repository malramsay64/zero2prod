-- Add migration script here
-- We don't want to allow for null values, however, for us to perform a zero
-- downtime migration, we need both the old and new version of the application
-- to run on the current version of the database. Having the null value allows
-- this, which we can fix in a later migration.
ALTER TABLE subscriptions ADD COLUMN status TEXT null;
