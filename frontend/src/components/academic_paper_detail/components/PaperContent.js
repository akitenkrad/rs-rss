import ReactMarkdown from 'react-markdown';
import rehypeHighlight from 'rehype-highlight';
import rehypeRaw from 'rehype-raw';
import remarkGfm from 'remark-gfm';

const PaperContent = ({ paper, isFullTextExpanded, onToggleFullText, CodeBlock }) => {
    return (
        <>
            <section className="detailed-content">
                <h2>Contents</h2>
                
                <div className="content-section">
                    <h3>Background & Research Question</h3>
                    <div className="section-divider"></div>
                    <p className="content-text">{paper.background_and_purpose}</p>
                </div>

                <div className="content-section">
                    <h3>Methodology</h3>
                    <div className="section-divider"></div>
                    <p className="content-text">{paper.methodology}</p>
                </div>

                <div className="content-section">
                    <h3>Dataset</h3>
                    <div className="section-divider"></div>
                    <p className="content-text">{paper.dataset}</p>
                </div>

                <div className="content-section">
                    <h3>Experiment Overview and Results</h3>
                    <div className="section-divider"></div>
                    <p className="content-text">{paper.results}</p>
                </div>

                <div className="content-section">
                    <h3>Future Works</h3>
                    <div className="section-divider"></div>
                    <p className="content-text">{paper.advantages_limitations_and_future_work}</p>
                </div>
            </section>

            <section className="full-text-section">
                <div className="full-text-header">
                    <h2>Full Text</h2>
                    <button 
                        className={`expand-button ${isFullTextExpanded ? 'expanded' : ''}`}
                        onClick={onToggleFullText}
                    >
                        {isFullTextExpanded ? 'Collapse' : 'Expand'} Full Text
                        <span className={`expand-icon ${isFullTextExpanded ? 'rotated' : ''}`}>▼</span>
                    </button>
                </div>
                <div className="full-text-divider"></div>
                
                <div className={`full-text-content ${isFullTextExpanded ? 'expanded' : 'collapsed'}`}>
                    {paper.text && typeof paper.text === 'string' && paper.text.length > 0 ? (
                        <div className="text-sections">
                            <div className="text-section">
                                <ReactMarkdown
                                    remarkPlugins={[remarkGfm]}
                                    rehypePlugins={[rehypeRaw, rehypeHighlight]}
                                    components={{
                                        code: CodeBlock
                                    }}
                                >
                                    {paper.text}
                                </ReactMarkdown>
                            </div>
                        </div>
                    ) : paper.text && Array.isArray(paper.text) && paper.text.length > 0 ? (
                        <div className="text-sections">
                            {paper.text.map((section, index) => (
                                <div key={index} className="text-section">
                                    <ReactMarkdown
                                        remarkPlugins={[remarkGfm]}
                                        rehypePlugins={[rehypeRaw, rehypeHighlight]}
                                        components={{
                                            code: CodeBlock
                                        }}
                                    >
                                        {section}
                                    </ReactMarkdown>
                                </div>
                            ))}
                        </div>
                    ) : (
                        <div className="no-text-message">
                            <p>Full text is not available for this paper.</p>
                        </div>
                    )}
                </div>
            </section>
        </>
    );
};

export default PaperContent;
