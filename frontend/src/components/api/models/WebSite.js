// WebSiteResponseに対応するクラス
export class WebSite {
    constructor({ site_id, name, url, ...rest }) {
        this.site_id = site_id;
        this.name = name;
        this.url = url;
        Object.assign(this, rest);
    }
}

// PaginatedWebSiteResponseに対応するクラス
export class PaginatedWebSiteResponse {
    constructor({ total, limit, offset, items = [], status_code, ...rest }) {
        this.total = total;
        this.limit = limit;
        this.offset = offset;
        this.items = items.map(item => new WebSite(item));
        this.status_code = status_code;
        Object.assign(this, rest);
    }
}
