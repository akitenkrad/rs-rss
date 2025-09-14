// PaperNoteResponseに対応するクラス
export class PaperNote {
    constructor({ paper_note_id, text, note_timestamp, ...rest }) {
        this.paper_note_id = paper_note_id;
        this.text = text;
        this.note_timestamp = note_timestamp;
        Object.assign(this, rest);
    }
}

// PaperNoteSelectResponseに対応するクラス
export class PaperNoteSelectResponse {
    constructor({ paper_notes = [], status_code, ...rest }) {
        this.paper_notes = paper_notes.map(note => new PaperNote(note));
        this.status_code = status_code;
        Object.assign(this, rest);
    }
}

// PaperNoteCreateResponseに対応するクラス
export class PaperNoteCreateResponse {
    constructor({ paper_note, status_code, ...rest }) {
        this.paper_note = paper_note ? new PaperNote(paper_note) : null;
        this.status_code = status_code;
        Object.assign(this, rest);
    }
}

// PaperNoteUpdateResponseに対応するクラス
export class PaperNoteUpdateResponse {
    constructor({ paper_note, status_code, ...rest }) {
        this.paper_note = paper_note ? new PaperNote(paper_note) : null;
        this.status_code = status_code;
        Object.assign(this, rest);
    }
}

// PaperNoteDeleteResponseに対応するクラス
export class PaperNoteDeleteResponse {
    constructor({ status_code, ...rest }) {
        this.status_code = status_code;
        Object.assign(this, rest);
    }
}