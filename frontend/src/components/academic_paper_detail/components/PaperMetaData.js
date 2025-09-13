
const PaperMetaData = ({ paper, onCopyBibtex, onKeywordClick, formatDate, formatNumber }) => {
    return (
        <section className="basic-info">
            <div className="authors-section">
                <h2>Authors</h2>
                <div className="authors-divider"></div>
                <div className="authors-grid">
                    {paper.authors.map((author, index) => (
                        <div key={index} className="author-card">
                            <a 
                                href={author.link || '#'} //TODO: Replace with actual author link
                                target="_blank" 
                                rel="noopener noreferrer"
                                className="author-link"
                            >
                                <span className="author-name">{author.name}</span>
                                <span className="author-h-index">H-Index: {author.h_index}</span>
                            </a>
                        </div>
                    ))}
                </div>
            </div>

            <div className="abstract-section">
                <h2>Abstract</h2>
                <div className="abstract-divider"></div>
                <p className="abstract-text">{paper.abstract_text}</p>
            </div>

            <div className="keywords-section">
                <h2>Keywords</h2>
                <div className="keywords-divider"></div>
                <div className="keywords-container">
                    {(paper.keywords || []).map((keyword, index) => (
                        <span 
                            key={index} 
                            className="keyword-tag"
                            onClick={() => onKeywordClick(keyword)}
                            tabIndex={0}
                            onKeyDown={(e) => {
                                if (e.key === 'Enter' || e.key === ' ') {
                                    e.preventDefault();
                                    onKeywordClick(keyword);
                                }
                            }}
                            role="button"
                            aria-label={`Search for papers with keyword: ${keyword}`}
                        >
                            <span className="keyword-icon">#</span>
                            {keyword}
                        </span>
                    ))}
                </div>
            </div>

            <div className="meta-data-section">
                <h2>Meta Data</h2>
                <div className="meta-data-divider"></div>
                
                <div className="meta-data-content">
                    <div className="meta-info-grid">
                        <div className="meta-info-item">
                            <span className="meta-label">Published Date</span>
                            <span className="meta-value published-date">{formatDate(paper.published_date)}</span>
                        </div>
                        <div className="meta-info-item">
                            <span className="meta-label">Journal</span>
                            <span className="meta-value journal">{paper.journal.name}</span>
                        </div>
                        <div className="meta-info-item">
                            <span className="meta-label">Primary Category</span>
                            <span className="meta-value primary-category">{paper.primary_category || 'N/A'}</span>
                        </div>
                        <div className="meta-info-item">
                            <span className="meta-label">Paper Link</span>
                            <a 
                                href={paper.url} 
                                target="_blank" 
                                rel="noopener noreferrer"
                                className="meta-value paper-link"
                            >
                                Read Paper
                            </a>
                        </div>
                    </div>
                    
                    <div className="citation-metrics-grid">
                        <div className="metric-card">
                            <span className="metric-label">Citation Count</span>
                            <span className="metric-value">{formatNumber(paper.citation_count)}</span>
                        </div>
                        <div className="metric-card">
                            <span className="metric-label">Reference Count</span>
                            <span className="metric-value">{formatNumber(paper.reference_count)}</span>
                        </div>
                        <div className="metric-card">
                            <span className="metric-label">Influential Citations</span>
                            <span className="metric-value">{formatNumber(paper.influential_citation_count)}</span>
                        </div>
                    </div>
                    
                    <div className="bibtex-section">
                        <span className="meta-label">Bibtex</span>
                        <div className="bibtex-container">
                            <pre className="bibtex-text">{paper.bibtex}</pre>
                            <button 
                                className="copy-bibtex-btn"
                                onClick={onCopyBibtex}
                                title="Copy Bibtex to clipboard"
                            >
                                📋 Copy
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    );
};

export default PaperMetaData;
