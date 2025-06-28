--1. drop triggers
DROP TRIGGER IF EXISTS author_set_updated_at_trigger ON author;
DROP TRIGGER IF EXISTS journal_set_updated_at_trigger ON journal;
DROP TRIGGER IF EXISTS task_set_updated_at_trigger ON task;
DROP TRIGGER IF EXISTS author_paper_relation_set_updated_at_trigger ON author_paper_relation;
DROP TRIGGER IF EXISTS task_paper_relation_set_updated_at_trigger ON task_paper_relation;
DROP TRIGGER IF EXISTS academic_paper_set_updated_at_trigger ON academic_paper;

--2. drop tables
DROP TABLE IF EXISTS author_paper_relation;
DROP TABLE IF EXISTS task_paper_relation;
DROP TABLE IF EXISTS academic_paper;
DROP TABLE IF EXISTS author;
DROP TABLE IF EXISTS journal;
DROP TABLE IF EXISTS task;
