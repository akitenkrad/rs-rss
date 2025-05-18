-- 1. create tables
CREATE TABLE IF NOT EXISTS author (
    author_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    ss_id VARCHAR(255) NOT NULL DEFAULT '',
    name VARCHAR(255) NOT NULL DEFAULT '',
    h_index INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS journal (
    journal_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    name VARCHAR(255) NOT NULL DEFAULT '',
    created_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TABLE IF NOT EXISTS task (
    task_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    task_name VARCHAR(255) NOT NULL DEFAULT '',
    created_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TABLE IF NOT EXISTS academic_paper (
    paper_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    ss_id VARCHAR(255) NOT NULL DEFAULT '',
    arxiv_id VARCHAR(255) NOT NULL DEFAULT '',
    journal_id UUID NOT NULL REFERENCES journal (journal_id),
    title TEXT NOT NULL DEFAULT '',
    abstract TEXT NOT NULL DEFAULT '',
    abstract_ja TEXT NOT NULL DEFAULT '',
    text TEXT NOT NULL DEFAULT '',
    url TEXT NOT NULL DEFAULT '',
    published_date DATE NOT NULL DEFAULT '1970-01-01',
    primary_category VARCHAR(255) NOT NULL DEFAULT '',
    citation_count INT NOT NULL DEFAULT 0,
    references_count INT NOT NULL DEFAULT 0,
    influential_citation_count INT NOT NULL DEFAULT 0,
    bibtex TEXT NOT NULL DEFAULT '',
    status_id UUID NOT NULL REFERENCES status (status_id),
    created_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TABLE IF NOT EXISTS author_paper_relation (
    author_id UUID NOT NULL REFERENCES author (author_id),
    paper_id UUID NOT NULL REFERENCES academic_paper (paper_id),
    PRIMARY KEY (author_id, paper_id)
);

CREATE TABLE IF NOT EXISTS task_paper_relation (
    task_id UUID NOT NULL REFERENCES task (task_id),
    paper_id UUID NOT NULL REFERENCES academic_paper (paper_id),
    PRIMARY KEY (task_id, paper_id)
);

-- 2. create triggers
CREATE OR REPLACE TRIGGER author_set_updated_at_trigger
    BEFORE UPDATE ON author
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

CREATE OR REPLACE TRIGGER journal_set_updated_at_trigger
    BEFORE UPDATE ON journal
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

CREATE OR REPLACE TRIGGER task_set_updated_at_trigger
    BEFORE UPDATE ON task
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

CREATE OR REPLACE TRIGGER author_paper_relation_set_updated_at_trigger
    BEFORE UPDATE ON author_paper_relation
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

CREATE OR REPLACE TRIGGER task_paper_relation_set_updated_at_trigger
    BEFORE UPDATE ON task_paper_relation
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

CREATE OR REPLACE TRIGGER academic_paper_set_updated_at_trigger
    BEFORE UPDATE ON academic_paper
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();