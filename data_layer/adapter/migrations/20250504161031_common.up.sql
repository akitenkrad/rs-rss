-- 1. define function
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
    BEGIN
        new.updated_at := ''now'';
        return new;
    END;
' LANGUAGE 'plpgsql';

-- 2. create tables
CREATE TABLE IF NOT EXISTS status (
    status_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    name VARCHAR(255) NOT NULL DEFAULT '',
    created_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3)
);

-- 3. create triggers
CREATE OR REPLACE TRIGGER status_set_updated_at_trigger
    BEFORE UPDATE ON status
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

-- 4. insert initial data
INSERT INTO
    status (name)
VALUES ('todo'),
    ('in progress'),
    ('in review'),
    ('completed'),
    ('deleted');