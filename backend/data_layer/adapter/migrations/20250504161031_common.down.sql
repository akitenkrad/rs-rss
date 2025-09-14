-- 1. drop triggers
DROP TRIGGER IF EXISTS web_article_set_updated_at_trigger ON web_article;

-- 2. drop tables
DROP TABLE IF EXISTS status;

-- 3. drop functions
DROP FUNCTION IF EXISTS set_updated_at;