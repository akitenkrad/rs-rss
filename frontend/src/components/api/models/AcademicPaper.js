// AuthorResponseに対応するクラス
export class Author {
    constructor({ author_id, ss_id, name, h_index }) {
        this.author_id = author_id;
        this.ss_id = ss_id;
        this.name = name;
        this.h_index = h_index;
    }
}

// TaskResponseに対応するクラス
export class Task {
    constructor({ task_id, name }) {
        this.task_id = task_id;
        this.name = name;
    }
}

// JournalResponseに対応するクラス
export class Journal {
    constructor({ journal_id, name }) {
        this.journal_id = journal_id;
        this.name = name;
    }
}

// AcademicPaperResponseに対応するクラス
export class AcademicPaper {
    constructor({
        paper_id,
        ss_id,
        arxiv_id,
        doi,
        title,
        abstract_text,
        authors = [],
        tasks = [],
        primary_category,
        published_date,
        created_at,
        updated_at,
        journal,
        text,
        url,
        citation_count,
        reference_count,
        influential_citation_count,
        bibtex,
        summary,
        background_and_purpose,
        methodology,
        dataset,
        results,
        advantages_limitations_and_future_work,
        status,
        ...rest
    }) {
        this.paper_id = paper_id;
        this.ss_id = ss_id;
        this.arxiv_id = arxiv_id;
        this.doi = doi;
        this.title = title;
        this.abstract_text = abstract_text;
        this.authors = authors.map(author => new Author(author));
        this.tasks = tasks.map(task => new Task(task));
        this.primary_category = primary_category;
        this.published_date = published_date;
        this.created_at = created_at;
        this.updated_at = updated_at;
        this.journal = journal ? new Journal(journal) : null;
        this.text = text;
        this.url = url;
        this.citation_count = citation_count;
        this.reference_count = reference_count;
        this.influential_citation_count = influential_citation_count;
        this.bibtex = bibtex;
        this.summary = summary;
        this.background_and_purpose = background_and_purpose;
        this.methodology = methodology;
        this.dataset = dataset;
        this.results = results;
        this.advantages_limitations_and_future_work = advantages_limitations_and_future_work;
        this.status = status;
        this.advantages_limitations_and_future_work = advantages_limitations_and_future_work;
        
        // その他のフィールドも柔軟に対応
        Object.assign(this, rest);
    }
}

// AcademicPaperListResponseに対応するクラス
export class AcademicPaperListResponse {
    constructor({ total, limit, offset, items = [], status_code, ...rest }) {
        this.total = total;
        this.limit = limit;
        this.offset = offset;
        this.items = items.map(item => new AcademicPaper(item));
        this.status_code = status_code;
        Object.assign(this, rest);
    }
}