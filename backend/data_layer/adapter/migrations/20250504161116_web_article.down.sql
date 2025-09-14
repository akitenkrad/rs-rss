--1. drop triggers
DROP TRIGGER IF EXISTS web_article_set_updated_at_trigger ON web_article;
DROP TRIGGER IF EXISTS web_site_set_updated_at_trigger ON web_site;

--2. drop tables
DROP TABLE IF EXISTS web_article;
DROP TABLE IF EXISTS web_site;
