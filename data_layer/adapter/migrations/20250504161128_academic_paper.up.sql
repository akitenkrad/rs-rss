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
    name VARCHAR(255) NOT NULL DEFAULT '',
    created_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TABLE IF NOT EXISTS keyword (
    keyword_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    category VARCHAR(255) NOT NULL DEFAULT '',
    name VARCHAR(255) NOT NULL DEFAULT '',
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
    doi TEXT NOT NULL DEFAULT '',
    published_date DATE NOT NULL DEFAULT '1970-01-01',
    primary_category VARCHAR(255) NOT NULL DEFAULT '',
    citation_count INT NOT NULL DEFAULT 0,
    references_count INT NOT NULL DEFAULT 0,
    influential_citation_count INT NOT NULL DEFAULT 0,
    bibtex TEXT NOT NULL DEFAULT '',
    summary TEXT NOT NULL DEFAULT '',
    background_and_purpose TEXT NOT NULL DEFAULT '',
    methodology TEXT NOT NULL DEFAULT '',
    dataset TEXT NOT NULL DEFAULT '',
    results TEXT NOT NULL DEFAULT '',
    advantages_limitations_and_future_work TEXT NOT NULL DEFAULT '',
    status_id UUID NOT NULL REFERENCES status (status_id),
    created_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TABLE IF NOT EXISTS paper_note (
    paper_note_id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    note TEXT NOT NULL DEFAULT '',
    note_timestamp TIMESTAMP(3) WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP(3),
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

CREATE TABLE IF NOT EXISTS paper_keyword_relation (
    paper_id UUID NOT NULL REFERENCES academic_paper (paper_id),
    keyword_id UUID NOT NULL REFERENCES keyword (keyword_id),
    PRIMARY KEY (paper_id, keyword_id)
);

CREATE TABLE IF NOT EXISTS paper_note_relation (
    paper_id UUID NOT NULL REFERENCES academic_paper (paper_id),
    paper_note_id UUID NOT NULL REFERENCES paper_note (paper_note_id),
    PRIMARY KEY (paper_id, paper_note_id)
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

CREATE OR REPLACE TRIGGER keyword_set_updated_at_trigger
    BEFORE UPDATE ON keyword
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

CREATE OR REPLACE TRIGGER paper_keyword_relation_set_updated_at_trigger
    BEFORE UPDATE ON paper_keyword_relation
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();

CREATE OR REPLACE TRIGGER paper_note_set_updated_at_trigger
    BEFORE UPDATE ON paper_note
    FOR EACH ROW
    EXECUTE FUNCTION set_updated_at();
