// WebArticleResponseに対応するクラス
export class WebArticle {
    constructor({
        site_id,
        site_name,
        site_url,
        article_id,
        title,
        description,
        url,
        text,
        html,
        timestamp,
        summary,
        is_new_technology_related,
        is_new_academic_paper_related,
        is_ai_related,
        is_it_related,
        is_new_product_related,
        is_security_related,
        status,
        ...rest
    }) {
        this.site_id = site_id;
        this.site_name = site_name;
        this.site_url = site_url;
        this.article_id = article_id;
        this.title = title;
        this.description = description;
        this.url = url;
        this.text = text;
        this.html = html;
        this.timestamp = timestamp;
        this.summary = summary;
        this.is_new_technology_related = is_new_technology_related;
        this.is_new_academic_paper_related = is_new_academic_paper_related;
        this.is_ai_related = is_ai_related;
        this.is_it_related = is_it_related;
        this.is_new_product_related = is_new_product_related;
        this.is_security_related = is_security_related;
        this.status = status;
        Object.assign(this, rest);
    }
}

// PaginatedWebArticleResponseに対応するクラス
export class PaginatedWebArticleResponse {
    constructor({ total, limit, offset, items = [], status_code, ...rest }) {
        this.total = total;
        this.limit = limit;
        this.offset = offset;
        this.items = items.map(item => new WebArticle(item));
        this.status_code = status_code;
        Object.assign(this, rest);
    }
}
