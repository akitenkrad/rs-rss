-- 1. create tables
CREATE TABLE IF NOT EXISTS web_site (
    site_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    name VARCHAR(255) NOT NULL DEFAULT '',
    url TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TABLE IF NOT EXISTS web_article (
    article_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    site_id UUID REFERENCES web_site (site_id),
    title TEXT NOT NULL DEFAULT '',
    description TEXT NOT NULL DEFAULT '',
    text TEXT NOT NULL DEFAULT '',
    html TEXT NOT NULL DEFAULT '',
    url VARCHAR NOT NULL DEFAULT '',
    timestamp DATE NOT NULL DEFAULT '1970-01-01',
    summary TEXT NOT NULL DEFAULT '',
    is_new_technology_related BOOLEAN NOT NULL DEFAULT FALSE,
    is_new_product_related BOOLEAN NOT NULL DEFAULT FALSE,
    is_new_academic_paper_related BOOLEAN NOT NULL DEFAULT FALSE,
    is_ai_related BOOLEAN NOT NULL DEFAULT FALSE,
    is_security_related BOOLEAN NOT NULL DEFAULT FALSE,
    is_it_related BOOLEAN NOT NULL DEFAULT FALSE,
    status_id UUID REFERENCES status (status_id),
    created_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3)
);

-- 2. create triggers
CREATE OR REPLACE TRIGGER web_article_set_updated_at_trigger
    BEFORE UPDATE ON web_article
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

CREATE OR REPLACE TRIGGER web_site_set_updated_at_trigger
    BEFORE UPDATE ON web_site
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();