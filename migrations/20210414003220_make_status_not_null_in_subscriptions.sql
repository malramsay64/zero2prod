-- We wrap the entire migration in a transaction to make sure it
-- successds or fails atomically.
BEGIN;
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status IS NULL;
    -- Make status a mandatory column
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;
